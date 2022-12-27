import { flow } from 'fp-ts/function';
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

const sum = A.reduce<O.Option<number>, number>(0, (acc, cur) => O.getOrElse(() => 0)(cur) + acc);
const max = A.reduce<number, number>(0, (acc, cur) => (acc > cur ? acc : cur));

export const solutionPart1 = flow(
  readFile,
  E.map(
    flow(
      S.split('\n\n'),
      readonlyArray.toArray,
      A.map(flow(S.split('\n'), readonlyArray.toArray, A.map(toNumber), sum)),
      max,
    ),
  ),
);
