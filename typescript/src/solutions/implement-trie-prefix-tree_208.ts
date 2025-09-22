/*
 * @lc app=leetcode id=208 lang=typescript
 *
 * [208] Implement Trie (Prefix Tree)
 */

// @lc code=start
type MyNode = {
  children: Record<string, MyNode>;
  isEnd: boolean;
};

class Trie {
  root: MyNode = {
    children: {},
    isEnd: false,
  };

  constructor() {}

  insert(word: string): void {
    let cursor = this.root;
    let index = 0;
    for (let c of word) {
      if (Object.keys(cursor.children).includes(c)) {
        cursor = cursor.children[c];
      } else {
        cursor.children[c] = {
          children: {},
          isEnd: false,
        };
        cursor = cursor.children[c];
      }
      if (!cursor.isEnd) {
        cursor.isEnd = index === word.length - 1;
      }
      index++;
    }
  }

  search(word: string): boolean {
    let cursor = this.root;
    for (let c of word) {
      if (!Object.keys(cursor.children).includes(c)) {
        return false;
      }
      cursor = cursor.children[c];
    }
    return cursor.isEnd;
  }

  startsWith(prefix: string): boolean {
    let cursor = this.root;
    for (let c of prefix) {
      if (!Object.keys(cursor.children).includes(c)) {
        return false;
      }
      cursor = cursor.children[c];
    }
    return true;
  }
}

/**
 * Your Trie object will be instantiated and called as such:
 * var obj = new Trie()
 * obj.insert(word)
 * var param_2 = obj.search(word)
 * var param_3 = obj.startsWith(prefix)
 */
// @lc code=end
