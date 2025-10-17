import type { SessionsResource } from "@repo/database/cache";
import type { Client, ParseClient } from "seyfert";
import type { context } from "./core/context";

declare module "seyfert" {
  interface UsingClient extends ParseClient<Client<true>> {
    readyAt: Date;
  }

  interface Cache {
    sessions: SessionsResource;
  }

  interface ExtendContext extends ReturnType<typeof context> {}
}

/**
 * Construct a type with the properties of T except for those in type K.
 */
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
