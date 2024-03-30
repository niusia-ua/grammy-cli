import { load } from "std:dotenv";
import { cleanEnv, str } from "envalid";

const raw = await load({
  examplePath: null,
  defaultsPath: null,
});
export const env = cleanEnv(raw, {
  BOT_TOKEN: str(),
});
