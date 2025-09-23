/*
 * @lc app=leetcode id=127 lang=typescript
 *
 * [127] Word Ladder
 */

// @lc code=start
function ladderLength(
  beginWord: string,
  endWord: string,
  wordList: string[]
): number {
  const set = new Set("abcdefghijklmnopqrstuvwxyz".split(""));
  const wordSet = new Set();
  const visited = new Set();
  const queue: {
    val: string;
    step: number;
  }[] = [];
  queue.push({
    val: beginWord,
    step: 1,
  });

  for (const w of wordList) {
    wordSet.add(w);
  }

  if (!wordSet.has(endWord)) {
    return 0;
  }

  while (queue.length) {
    const item = queue.shift()!;
    for (let i = 0; i < item.val.length; i++) {
      for (const c of set) {
        const newWord =
          item.val.slice(0, i) + c + item.val.slice(i + 1, item.val.length);
        if (newWord === endWord) {
          return item.step + 1;
        } else if (wordSet.has(newWord) && !visited.has(newWord)) {
          queue.push({
            val: newWord,
            step: item.step + 1,
          });
          visited.add(newWord);
        }
      }
    }
  }

  return 0;
}
// @lc code=end
