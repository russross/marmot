String.prototype.hashCode = function() {
  var hash = 0,
    i, chr;
  if (this.length === 0) return hash;
  for (j = 0; j < 4; j++)
      for (i = 0; i < this.length; i++) {
        chr = this.charCodeAt(i);
        hash = ((hash << 5) - hash) + chr;
        hash |= 0; // Convert to 32bit integer
      }
  return hash;
};

window.addEventListener('load', function () {
    let schedule = document.getElementById('schedule');
    let days_to_show = ['M', 'T', 'W', 'R', 'F'];

    let allPrefixes = [];
    let penaltiesByPrefix = {};
    let totalScoreByPrefix = {};
    (function (sched) {
        for (section of sched) {
            for (elt of section.prefixes) {
                if (!allPrefixes.includes(elt)) {
                    allPrefixes.push(elt);
                }

                // gather penalties by prefix
                if (!(elt in penaltiesByPrefix)) {
                    penaltiesByPrefix[elt] = [];
                }
                if (!(elt in totalScoreByPrefix)) {
                    totalScoreByPrefix[elt] = 0;
                }
                for (problem of section.problems) {
                    penaltiesByPrefix[elt].push(problem);
                    totalScoreByPrefix[elt] += problem.score;
                }
            }
        }
        allPrefixes.sort(function (a, b) {
            if (a.hashCode() < b.hashCode()) return -1;
            if (a.hashCode() > b.hashCode()) return 1;
            return 0;
        });
        for (prefix of allPrefixes) {
            let list = penaltiesByPrefix[prefix];
            list.sort(function (a, b) {
                if (a.score > b.score) return -1;
                if (a.score < b.score) return 1;
                if (a.message < b.message) return -1;
                if (a.message > b.message) return 1;
                return 0;
            });
        }
    })(window.placement);

    let build_room_time_grid = function (rooms, days) {
        schedule.style.setProperty('--grid-columns', rooms.length);
        for (room of rooms) {
            let span = document.createElement('div');
            schedule.appendChild(span);
            span.classList.add('room-name');
            span.style.gridColumn = rooms.indexOf(room) + 2;
            span.innerText = room;
        }

        let rows = 1;
        let row_key = {};
        for (day of days) {
            let letter = day[0];
            let start_minutes = day[1];
            let end_minutes = day[2];

            for (let i = start_minutes; i < end_minutes; i += 5) {
                row_key[letter + i] = ++rows;
                let hour = Math.floor(i / 60);
                let minute = i % 60;
                let am = hour < 12 ? 'am' : 'pm';
                hour %= 12;
                if (hour == 0) hour = 12;
                if (minute == 0) {
                    let h2 = document.createElement('div');
                    schedule.appendChild(h2);
                    h2.classList.add('time-name');
                    let m = minute < 10 ? '0' + minute : '' + minute;
                    h2.innerHTML = letter + '&nbsp;' + hour + ':' + m + '&nbsp;' + am;
                    h2.style.gridRow = '' + rows + '/ span 12';
                }
            }
        }
        schedule.style.setProperty('--grid-rows', rows-1);

        return row_key;
    };

    const time_slot_re = /^([mtwrfsuMTWRFSU]+)([0-1][0-9]|2[0-3])([0-5][05])\+([1-9][0-9]?[05])$/;
    let parse_time_slot = function (time_slot) {
        let parts = time_slot_re.exec(time_slot);
        let days = parts[1];
        let start = Number(parts[2])*60 + Number(parts[3]);
        let duration = Number(parts[4]);
        let end = start + duration;
        let start_hour = Math.floor(start / 60);
        let start_minute = start % 60;
        let start_am = start_hour < 12 ? 'am' : 'pm';
        start_hour %= 12;
        if (start_hour == 0) start_hour = 12;
        let sm = (start_minute < 10 ? '0' : '') + start_minute;
        let end_hour = Math.floor(end / 60);
        let end_minute = end % 60;
        let end_am = end_hour < 12 ? 'am' : 'pm';
        end_hour %= 12;
        if (end_hour == 0) end_hour = 12;
        let em = (end_minute < 10 ? '0' : '') + end_minute;
        let sam = start_am == end_am ? '' : ' ' + start_am;
        let start_label = `${start_hour}:${sm} ${start_am}`;
        let end_label = `${end_hour}:${em} ${end_am}`;
        let range_label = `${start_hour}:${sm}${sam}&ndash;${end_hour}:${em} ${end_am}`;
        return {
            time_slot: time_slot,
            days: days.split(''),
            start_minutes: start,
            start_hour: start_hour,
            start_minute: start_minute,
            start_am: start_am,
            duration: duration,
            end_minutes: end,
            end_hour: end_hour,
            end_minute: end_minute,
            end_am: end_am,
            start_label: start_label,
            end_label: end_label,
            range_label: range_label,
        };
    };

    let find_time_range = function (days, sections) {
        let by_day = {};
        for (section of sections) {
            let time = parse_time_slot(section.time_slot);
            for (day of time.days) {
                if (!days.includes(day)) continue;
                let range = by_day[day] || [time.start_minutes, time.end_minutes];
                range[0] = Math.min(range[0], time.start_minutes);
                range[1] = Math.max(range[1], time.end_minutes);
                by_day[day] = range;
            }
        }
        let result = [];
        for (day of days) {
            let range = by_day[day];
            if (!range) continue;
            let start = range[0] - range[0]%60;
            let end = range[1]+59;
            end = end - end%60;
            result.push([day, start, end]);
        }
        return result;
    };

    let sections = [];
    let rooms = [];
    prefixes = [];
    for (elt of window.placement) {
        //if (elt.is_placed && (elt.room.startsWith('SNOW ') || elt.room.startsWith('SET ') || elt.room.startsWith('Smith '))) {
        if (elt.is_placed && (['SET 105', 'SET 106', 'SET 201', 'SET 301', 'SET 418', 'SET 420', 'SET 524'].includes(elt.room))) {
            sections.push(elt);
            for (prefix of elt.prefixes)
                if (!prefixes.includes(prefix))
                    prefixes.push(prefix);
            if (!rooms.includes(elt.room))
                rooms.push(elt.room);
        }
    }
    prefixes.sort(function (a, b) {
        if (a.hashCode() < b.hashCode()) return -1;
        if (a.hashCode() > b.hashCode()) return 1;
        return 0;
    });
    rooms.sort();
    let row_key = build_room_time_grid(rooms, find_time_range(days_to_show, sections));

    let make_section = function (elt) {
        let box = document.createElement('div');
        box.classList.add('section');

        let h = Math.round(320 * (prefixes.indexOf(section.prefixes[0]) + 0.0) / prefixes.length + 145) % 360;
        let color = 'lch(var(--l) var(--c) ' + h + ')';

        box.style.backgroundColor = color;

        let name = document.createElement('h3');
        box.appendChild(name);
        name.classList.add('section-name');

        let s = '';
        let sep = '';
        for (elt of section.names) {
            s += sep + elt;
            sep = ' / ';
        }
        name.innerText = s;

        let time = document.createElement('span');
        box.appendChild(time);
        time.classList.add('section-time');
        time.innerHTML = parse_time_slot(section.time_slot).range_label;

        if (section.instructors.length > 0) {
            let instructor = document.createElement('span');
            box.appendChild(instructor);
            instructor.classList.add('section-instructor');
            sep = '';
            s = '';
            for (elt of section.instructors) {
                s += sep + elt.replace(/\s+/, '&nbsp;');
                sep = ' and ';
            }

            instructor.innerHTML = s;
        }

        if (section.problems.length > 0) {
            box.style.border = 'dashed 4px black';
            let problems = document.createElement('div');
            box.appendChild(problems);
            let title = '';
            let br = '';
            let score = 0;
            for (p of section.problems) {
                score += p.score;
                title += br + '' + p.score + ': ' + p.message;
                br = '\n';
            }
            problems.innerText = 'score: ' + score;
            box.title = title;
        }

        //let hue = document.createElement('span');
        //box.appendChild(hue);
        //hue.innerText = color;

        return box;
    };
    for (section of sections) {
        for (day of days_to_show) {
            let time = parse_time_slot(section.time_slot);
            if (!time.days.includes(day)) continue;

            let box = make_section(section);
            schedule.appendChild(box);

            box.style.gridColumn = rooms.indexOf(section.room) + 2;
            let key = day + time.start_minutes;
            box.style.gridRow = '' + row_key[day+time.start_minutes] + ' / span ' + (time.duration)/5;
        }
    }

    // report all the penalties in a list at the bottom
    let penalty_div = document.getElementById('penalties');
    for (prefix of allPrefixes.toSorted()) {
        let h2 = document.createElement('h2');
        penalty_div.appendChild(h2);
        h2.innerText = 'Penalties for ' + prefix + ' (total ' + totalScoreByPrefix[prefix] + ')';
        let penalties = [];
        let ul = document.createElement('ul');
        penalty_div.appendChild(ul);
        let prev_score = -1;
        for (problem of penaltiesByPrefix[prefix]) {
            let li = document.createElement('li');
            ul.appendChild(li);
            let padded = '' + problem.score;
            if (padded.length < 2) padded = ' ' + padded;
            let msg = '[' + padded + '] ' + problem.message;
            li.innerText = msg;

            if (problem.score != prev_score) {
                prev_score = problem.score;
                penalties.push([problem.score, 0]);
            }
            penalties[penalties.length-1][1]++;
        }

        let h3 = document.createElement('h3');
        penalty_div.appendChild(h3);
        h3.innerText = 'Penalty counts by score for ' + prefix;
        ul = document.createElement('ul');
        penalty_div.appendChild(ul);
        for (score of penalties) {
            li = document.createElement('li');
            ul.appendChild(li);
            let padded = '' + score[0];
            if (padded.length < 2) padded = ' ' + padded;
            let pen = 'penalties';
            if (score[1] == 1) pen = 'penalty';
            let msg = '[' + padded + '] ' + score[1] + ' ' + pen;
            li.innerText = msg;
        }
    }
});;
