import { readFileSync } from "fs";
import path from "path";

const expected = {
  red: 12,
  green: 13,
  blue: 14,
};
const part1 = (filepath: string) => {
  return readFileSync(filepath, "utf8")
    .split("\n")
    .map((line) => {
      const [game, roundsStr] = line.split(":");
      const gameId = Number(game.replace("Game ", "").trim());
      const rounds = roundsStr
        .split(";")
        .map((round) =>
          round.split(",").map((cube) => {
            const [num, color] = cube.trim().split(" ");
            return { num: Number(num), color };
          })
        )
        .flat();

      return { gameId, rounds };
    })
    .filter((game) => {
      return game.rounds.every(
        //@ts-ignore
        (round) => round.num <= expected?.[round.color] ?? 0
      );
    })
    .reduce((acc, game) => acc + game.gameId, 0);
};

const part2 = (filepath: string) => {
  return readFileSync(filepath, "utf8")
    .split("\n")
    .map((line) => {
      const [game, roundsStr] = line.split(":");
      const gameId = Number(game.replace("Game ", "").trim());
      const rounds = roundsStr
        .split(";")
        .map((round) =>
          round.split(",").map((cube) => {
            const [num, color] = cube.trim().split(" ");
            return { num: Number(num), color };
          })
        )
        .flat()
        .sort((a, b) => b.num - a.num)
        .filter(
          (cube, index, arr) =>
            arr.findIndex((c, i) => c.color === cube.color) === index
        )
        .reduce((acc, cube) => acc * cube.num, 1);

      return { gameId, rounds };
    })
    .reduce((acc, game) => acc + game.rounds, 0);
};

const run = (fn: (filepath: string) => any) => {
  const fnName = fn.name;
  const examplePath = path.join(__dirname, "day-02-example.txt");
  const inputPath = path.join(__dirname, "day-02.txt");

  console.log(`${fnName} example: ${fn(examplePath)}`);
  console.log(`${fnName} input: ${fn(inputPath)}`);
};

run(part1);
run(part2);
