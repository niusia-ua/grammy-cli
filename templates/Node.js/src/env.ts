import { cleanEnv, str } from "envalid";

export const env = cleanEnv(process.env, {
  BOT_TOKEN: str(),
});
