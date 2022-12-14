import { describe, it, expect } from 'vitest';
import { solutionPart1 } from './day-01';
import path from 'node:path';
import * as E from 'fp-ts/Either';
import { solution2Part1 } from './day-01 copy';

describe('Challenge day-01/2022 with fp-ts', () => {
  it('highest calories', async () => {
    const filepath = path.resolve(__dirname, '..', 'input.txt');

    expect(await solutionPart1(filepath)).toEqual(E.right(24000));
  });

  it('highest calories solution 2', async () => {
    const filepath = path.resolve(__dirname, '..', 'input.txt');

    expect(await solution2Part1(filepath)).toEqual(E.right(24000));
  });
});
