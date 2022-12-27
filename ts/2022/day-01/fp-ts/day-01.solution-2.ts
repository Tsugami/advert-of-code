import { flow, pipe } from 'fp-ts/function';
import * as E from 'fp-ts/Either';
import * as O from 'fp-ts/Option';
import * as S from 'fp-ts/string';
import * as A from 'fp-ts/Array';

import fs from 'node:fs';
import { readonlyArray } from 'fp-ts';

const readFile = E.tryCatchK((filepath: string) => fs.readFileSync(filepath, 'utf-8'), E.toError);

const toNumber = (s: string) => {
  const num = Number(s);
  return isNaN(num) ? O.none : O.some(num);
};

const sum = A.reduce<string, number>(
  0,
  (acc, cur) =>
    pipe(
      cur,
      toNumber,
      O.getOrElse(() => 0),
    ) + acc,
);

export const solution2Part1 = flow(
  readFile,
  E.map(
    flow(
      S.split('\n\n'),
      readonlyArray.toArray,
      A.reduce(0, (acc, cur) => {
        const val = pipe(cur, S.split('\n'), readonlyArray.toArray, sum);
        return val > acc ? val : acc;
      }),
    ),
  ),
);
