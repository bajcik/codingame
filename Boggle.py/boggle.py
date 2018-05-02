import sys

class Grid:
    def __init__(self, lines):
        self.lines = lines
        self.used = [[False, False, False, False] for x in range(4)]
    
    def has_word_from(self, word, x, y):
        if len(word) == 0:
            return True
        
        if x < 0 or x > 3: return False
        if y < 0 or y > 3: return False
        if self.used[y][x]: return False
        if self.lines[y][x] != word[0]: return False
        
        self.used[y][x] = True
        found = False
        for direct in ((0,1),(1,1),(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1)):
            dx,dy = direct
            x2 = x+dx
            y2 = y+dy
            found = self.has_word_from(word[1:], x2, y2)
            if found:
                break
        self.used[y][x] = False
        return found
    
    def has_word(self, word):
        for y in range(4):
            for x in range(4):
                if self.has_word_from(word, x, y):
                    return True
        return False

grid = Grid([input(), input(), input(), input()])
n = int(input())
for i in range(n):
    word = input()
    print("true" if grid.has_word(word) else "false" )


