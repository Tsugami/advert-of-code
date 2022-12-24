import fs from "node:fs";
import path from "node:path";

function isNumber(v) {
  return !Number.isNaN(Number(v));
}

export function processFile(filepath) {
  const file = fs.readFileSync(path.resolve(filepath), "utf-8");

  const [gridText, procedureText] = file.split("\n\n");

  const procedures = parseProcedureTextToProcedureObjectArray(procedureText);
  const stacks = parseGridTextToCrateStacks(gridText);

  const part1Result = processProcedures(
    cloneDepthArray(stacks),
    procedures,
    true
  );

  console.log(`${filepath} - result part-1: ${part1Result}`);

  const part2Result = processProcedures(
    cloneDepthArray(stacks),
    procedures,
    false
  );

  console.log(`${filepath} - result part-2: ${part2Result}`);
}

export function parseProcedureTextToProcedureObjectArray(procedureText) {
  const procedureLines = procedureText.split("\n");

  return procedureLines.map((line_) => {
    const line = line_.trim();
    const REGEX = /move\s(\d*)\sfrom\s(\d*)\sto\s(\d*)$/;

    const result = line.match(REGEX);

    if (!result) {
      throw new Error("formato de procedimento está invalido:" + line);
    }

    const [_input, quantity, from, to] = result;

    return { quantity: Number(quantity), from: Number(from), to: Number(to) };
  });
}

export function parseGridTextToCrateStacks(gridText) {
  const stacks = gridText.split("\n");
  stacks.pop();

  const grid = [];

  for (const line of stacks) {
    const chars = line.split("");

    let i = 0;
    while (chars.length > 0) {
      const [_, crate] = chars.splice(0, 4);

      if (crate !== " ") {
        if (grid[i]) {
          grid[i].push(crate);
        } else {
          grid[i] = [crate];
        }
      }

      i++;
    }
  }

  return grid;
}

export function processProcedures(stacks, procedures, moveOneByOne) {
  for (const procedure of procedures) {
    const letters = stacks[procedure.from - 1].splice(0, procedure.quantity);

    if (moveOneByOne) {
      letters.reverse();
    }

    stacks[procedure.to - 1].unshift(...letters);
  }

  return stacks.map((coll) => coll[0] ?? "").join("");
}

function cloneDepthArray(arr) {
  return Array.isArray(arr) ? [...arr.map(cloneDepthArray)] : arr;
}
