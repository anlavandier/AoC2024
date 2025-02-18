import re

pat = re.compile(r"mul\((?P<first_op>\d{1,3}),(?P<scnd_op>\d{1,3})\)")

input = open("input.txt").read()

matches = list(re.finditer(pat, input))

sum_of_prods = sum(int(m["first_op"]) * int(m["scnd_op"]) for m in matches)

print(sum_of_prods)

dos_pat = re.compile(r"(do|don't)\(\)")
dos_pos = dos_pat.finditer(input)

valid_spans = []
cur_dont = False
cur_span = (0,)

for do_match in dos_pos:
    if cur_dont:
        if do_match.group() == 'do()':
            cur_span = (do_match.span()[1],)
            cur_dont = False
    else:
        if do_match.group() == "don't()":
            cur_span = cur_span + (do_match.span()[0],)
            valid_spans.append(cur_span)
            cur_dont = True
if not cur_dont:
    valid_spans.append(cur_span + (len(input),))
sum_of_valid = sum(int(m["first_op"]) * int(m["scnd_op"]) 
                   for m in matches 
                   if any(l < m.span()[1] <= g for l, g in valid_spans))

print(valid_spans)

print(sum_of_valid)