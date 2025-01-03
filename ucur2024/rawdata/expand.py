import re
import sys

mappings = {
    "2 hour lab M0800": [ "M0800+110", "M1000+110", "M1200+110", "M1400+110", "M1600+110" ],
    "2 hour lab M0900": [ "M0900+110", "M1100+110", "M1300+110", "M1500+110" ],
    "2 hour lab T0800": [ "T0800+110", "T1000+110", "T1200+110", "T1400+110", "T1600+110" ],
    "2 hour lab T0900": [ "T0900+110", "T1100+110", "T1300+110", "T1500+110" ],
    "2 hour lab W0800": [ "W0800+110", "W1000+110", "W1200+110", "W1400+110", "W1600+110" ],
    "2 hour lab W0900": [ "W0900+110", "W1100+110", "W1300+110", "W1500+110" ],
    "2 hour lab R0800": [ "R0800+110", "R1000+110", "R1200+110", "R1400+110", "R1600+110" ],
    "2 hour lab R0900": [ "R0900+110", "R1100+110", "R1300+110", "R1500+110" ],

    "3 hour lab M0800": [ "M0800+170", "M1100+170", "M1400+170" ],
    "3 hour lab M0900": [ "M0900+170", "M1200+170", "M1500+170" ],
    "3 hour lab M1000": [ "M1000+170", "M1300+170" ],
    "3 hour lab T0800": [ "T0800+170", "T1100+170", "T1400+170" ],
    "3 hour lab T0900": [ "T0900+170", "T1200+170", "T1500+170" ],
    "3 hour lab T1000": [ "T1000+170", "T1300+170" ],
    "3 hour lab W0800": [ "W0800+170", "W1100+170", "W1400+170" ],
    "3 hour lab W0900": [ "W0900+170", "W1200+170", "W1500+170" ],
    "3 hour lab W1000": [ "W1000+170", "W1300+170" ],
    "3 hour lab R0800": [ "R0800+170", "R1100+170", "R1400+170" ],
    "3 hour lab R0900": [ "R0900+170", "R1200+170", "R1500+170" ],
    "3 hour lab R1000": [ "R1000+170", "R1300+170" ],

    "4 hour lab MW0800": [ "MW0800+110", "MW1000+110", "MW1200+110", "MW1400+110", "MW1600+110" ],
    "4 hour lab MW0900": [ "MW0900+110", "MW1100+110", "MW1300+110", "MW1500+110" ],
    "4 hour lab TR0800": [ "TR0800+110", "TR1000+110", "TR1200+110", "TR1400+110", "TR1600+110" ],
    "4 hour lab TR0900": [ "TR0900+110", "TR1100+110", "TR1300+110", "TR1500+110" ],
}

sequence = -1
room = 'NOT A ROOM'
replacement_tag = 'NOT A TAG'
start_line = r'assigned to (\S+ \S+) at ([A-Z]+\d+\+\d+)$'
for line in sys.stdin:
    if sequence >= 0:
        if sequence == 1 and line.find('section!(t, course: ') >= 0:
            sequence = 2
        elif sequence == 2 and line.find('instructor: ') >= 0:
            sequence = 3
        elif sequence in (2, 3) and line.find('rooms and times:') >= 0:
            sequence = 4
        elif sequence == 4:
            if line.find(f'"{room}"') < 0:
                print('expected room to match')
                sys.exit(1)
            sequence = 5
        elif sequence == 5:
            line = ' '*20 + f'"{replacement_tag}",'
            sequence = 6
        elif sequence == 6 and line.find(');') >= 0:
            sequence = -1
        else:
            print('broken sequence')
            sys.exit(1)
        print(line[:-1])
        continue

    handled = False
    for (tag, actual_slots) in mappings.items():
        if handled: break
        for slot in actual_slots:
            if handled: break
            if line.find(f'"{slot}"') >= 0 and line.find('hour lab"') >= 0:
                line = line.replace('hour lab"', f'{tag[2:]}"')
                handled = True
            elif line.find(f'name: "{slot}");') >= 0:
                line = line.replace(');', f', tags: "{tag}");')
                handled = True
            elif re.search(start_line, line):
                if sequence >= 0:
                    print('unexpected assigned to line in the middle of a section')
                    sys.exit(1)
                groups = re.search(start_line, line)
                (room, time_slot) = (groups[1], groups[2])
                if time_slot == slot:
                    sequence = 1
                    replacement_tag = tag
                    handled = True
                else:
                    continue
            else:
                continue
    print(line[:-1])
