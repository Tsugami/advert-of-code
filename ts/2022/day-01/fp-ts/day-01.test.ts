import { describe, it, expect } from 'vitest';
import { solutionPart1 } from './day-01';
import path from 'node:path';
import * as E from 'fp-ts/Either';

describe('Challenge day-01/2022 with fp-ts', () => {
  it('highest calories', async () => {
    const filepath = path.resolve(__dirname, '..', 'input.txt');

    expect(await solutionPart1(filepath)).toEqual(E.right(24000));
  });
});
