import { processFile } from "./challenge.js";

import { fileURLToPath } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

processFile(path.resolve(__dirname, "input", "example.txt")); // CMZ - MCD
processFile(path.resolve(__dirname, "input", "input.txt"));
