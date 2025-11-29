def escape(s: str):
  return s.replace('\n', '\\n').replace('\\', '\\\\').replace('\"', '\\\"')

string = ' < \"What are\\\n you\\\" doing?\">'
length = len(string)

in_str = False
index_a = 0
index_b = 0
while index_a < length:
  char = string[index_a]

  if char == '\"':
    if index_a != 0:
      is_esc = string[index_a-1]
      if is_esc == '\\':
        index_a += 1
        continue

    in_str = not in_str
    if not in_str:
      substr = string[index_b+1:index_a]
      print(f'OK: {escape(substr)}')

    index_b = index_a
  index_a += 1