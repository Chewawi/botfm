import {
	ActionRow,
	type AnyContext,
	Button,
	type ButtonInteraction,
	type Container,
	type Message,
	Modal,
	type ModalSubmitInteraction,
	TextInput,
	type WebhookMessage,
} from "seyfert";
import { Label } from "seyfert/lib/builders/Label";
import type { CreateComponentCollectorResult } from "seyfert/lib/components/handler.js";
import { ButtonStyle, MessageFlags, TextInputStyle } from "seyfert/lib/types";

interface PaginatorOptions {
	ctx: AnyContext;
	containers: Container[];
	time?: number;
}

export class Paginator {
	private readonly ctx: AnyContext;
	private readonly containers: Container[];
	private readonly time: number;
	private currentPage: number = 0;
	private message: Message | WebhookMessage | null = null;

	constructor(options: PaginatorOptions) {
		this.ctx = options.ctx;
		this.containers = options.containers;
		this.time = options.time ?? 60000;
	}

	private getButtons(disabled: boolean = false): ActionRow[] {
		return [
			new ActionRow().addComponents(
				new Button()
					.setEmoji("<:back:1428853176781508709>")
					.setStyle(ButtonStyle.Secondary)
					.setCustomId("paginator_prev")
					.setDisabled(disabled || this.currentPage === 0),
				new Button()
					.setEmoji("<:forward:1428853180426621038>")
					.setStyle(ButtonStyle.Secondary)
					.setCustomId("paginator_next")
					.setDisabled(
						disabled || this.currentPage === this.containers.length - 1,
					),
				new Button()
					.setEmoji("<:shuffle:1428854202880491591>")
					.setStyle(ButtonStyle.Secondary)
					.setCustomId("paginator_random")
					.setDisabled(disabled || this.containers.length <= 1),
				new Button()
					.setEmoji("<:page:1428854201030676591>")
					.setStyle(ButtonStyle.Secondary)
					.setCustomId("paginator_goto")
					.setDisabled(disabled || this.containers.length <= 1),
				new Button()
					.setEmoji("<:delete:1428855032849367040>")
					.setStyle(ButtonStyle.Danger)
					.setCustomId("paginator_delete")
					.setDisabled(disabled),
			),
		];
	}

	async reply(ephemeral: boolean = false): Promise<void> {
		if (!this.containers.length) {
			throw new Error("Cannot send paginator without containers");
		}

		const currentContainer = this.containers[this.currentPage];
		if (!currentContainer) {
			throw new Error("Invalid page index");
		}

		this.message = await this.ctx.editOrReply(
			{
				components: [currentContainer, ...this.getButtons()],
				flags: ephemeral
					? MessageFlags.Ephemeral | MessageFlags.IsComponentsV2
					: MessageFlags.IsComponentsV2,
			},
			true,
		);

		const collector: CreateComponentCollectorResult =
			this.message.createComponentCollector({
				idle: this.time,
				filter: (interaction) => interaction.user.id === this.ctx.author.id,
				onPass: async (interaction) => {
					await interaction.write({
						flags: MessageFlags.Ephemeral,
						content: `Only <@${this.ctx.author.id}> can use this paginator.`,
					});
				},
				onStop: async (reason) => {
					if (this.message && reason === "idle") {
						const container = this.containers[this.currentPage];
						if (!container) return;

						await this.ctx
							.editOrReply({
								components: [container, ...this.getButtons(true)],
								flags: MessageFlags.IsComponentsV2,
							})
							.catch(() => null);
					}
				},
			});

		// Navigation buttons
		collector.run<ButtonInteraction>(
			["paginator_prev", "paginator_next"],
			async (interaction) => {
				if (!interaction.isButton()) return;

				if (interaction.customId === "paginator_prev" && this.currentPage > 0) {
					this.currentPage--;
				} else if (
					interaction.customId === "paginator_next" &&
					this.currentPage < this.containers.length - 1
				) {
					this.currentPage++;
				}

				await interaction.deferUpdate();
				await this.update();
			},
		);

		// Random page button
		collector.run<ButtonInteraction>(
			"paginator_random",
			async (interaction) => {
				if (!interaction.isButton()) return;

				const randomPage = Math.floor(Math.random() * this.containers.length);
				this.currentPage = randomPage;

				await interaction.deferUpdate();
				await this.update();
			},
		);

		// Go to page button
		collector.run<ButtonInteraction>("paginator_goto", async (interaction) => {
			if (!interaction.isButton()) return;

			const pageInput = new Label().setLabel("Page Number").setComponent(
				new TextInput()
					.setCustomId("page_number")
					.setStyle(TextInputStyle.Short)
					.setPlaceholder(
						`Enter a number between 1 and ${this.containers.length}`,
					)
					.setRequired(true)
					.setLength({ min: 1, max: String(this.containers.length).length }),
			);

			const modal = new Modal()
				.setCustomId("paginator_goto_modal")
				.setTitle("Go to Page")
				.setComponents([pageInput])
				.run(async (modalInteraction: ModalSubmitInteraction) => {
					if (modalInteraction.user.id !== this.ctx.author.id) {
						return modalInteraction.write({
							flags: MessageFlags.Ephemeral,
							content: `Only <@${this.ctx.author.id}> can use this paginator.`,
						});
					}

					const pageValue = modalInteraction.getInputValue("page_number", true);
					const pageNumber = Number.parseInt(pageValue as string, 10);

					if (
						Number.isNaN(pageNumber) ||
						pageNumber < 1 ||
						pageNumber > this.containers.length
					) {
						return modalInteraction.write({
							flags: MessageFlags.Ephemeral,
							content: `Invalid page number. Please enter a number between 1 and ${this.containers.length}.`,
						});
					}

					this.currentPage = pageNumber - 1;
					await modalInteraction.deferUpdate();
					await this.update();
				});

			await interaction.modal(modal);
		});

		// Delete button
		collector.run<ButtonInteraction>(
			"paginator_delete",
			async (interaction) => {
				if (!interaction.isButton()) return;

				await interaction.deferUpdate();

				// Delete the paginator message
				if (this.message) {
					await this.message.delete().catch(() => null);
				}

				// Try to delete the original command message if it exists
				try {
					const response = await this.ctx.fetchResponse();

					// This shit doesn't work
					if (response.referencedMessage?.id) {
						const message = await this.ctx.client.messages.fetch(
							response.referencedMessage.id,
							this.ctx.channelId,
							true,
						);
						await message.delete().catch(() => null);
					}
				} catch {
					// Ignore errors
				}

				collector.stop("deleted");
			},
		);
	}

	private async update(): Promise<void> {
		if (!this.message) return;

		const currentContainer = this.containers[this.currentPage];
		if (!currentContainer) return;

		await this.ctx
			.editOrReply({
				components: [currentContainer, ...this.getButtons()],
				flags: MessageFlags.IsComponentsV2,
			})
			.catch(() => null);
	}
}
