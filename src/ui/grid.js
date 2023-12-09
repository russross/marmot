window.addEventListener('load', function () {
    let schedule = document.getElementById('schedule');

    let hsv_to_rgb = function (h, s, v) {
        let i = Math.floor(h * 6);
        let f = h * 6 - i;
        let p = v * (1 - s);
        let q = v * (1 - f * s);
        let t = v * (1 - (1 - f) * s);
        let r, g, b;
        switch (i % 6) {
            case 0: r = v, g = t, b = p; break;
            case 1: r = q, g = v, b = p; break;
            case 2: r = p, g = v, b = t; break;
            case 3: r = p, g = q, b = v; break;
            case 4: r = t, g = p, b = v; break;
            case 5: r = v, g = p, b = q; break;
        }
        return 'rgb(' +
            Math.round(r*255) + ',' + Math.round(g*255) + ',' +  Math.round(b*255) +
            ')';
    };

    let hash = function (s) {
        s = s + s + s + s;
        let hash = 0;
        for (const ch of s) {
            hash = (((hash<<5) - hash) + ch.charCodeAt(0)) % 0x10000;
        }
        return hash / 0x10000;
    };

    let build_room_time_grid = function (rooms, days) {
        console.log(rooms.length + 1);
        schedule.style.setProperty('--grid-columns', rooms.length + 1);
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
                    let h2 = document.createElement('h2');
                    schedule.appendChild(h2);
                    h2.classList.add('time-name');
                    let m = minute < 10 ? '0' + minute : '' + minute;
                    h2.innerHTML = letter + '&nbsp;' + hour + ':' + m + '&nbsp;' + am;
                    h2.style.gridRow = rows;
                }
            }
        }
        schedule.style.setProperty('--grid-rows', rows);
        console.log(rows);

        return row_key;
    };

    let sections = [];
    let rooms = [];
    for (elt of window.placements) {
        if (elt.is_placed && (elt.room.startsWith('SNOW ') || elt.room.startsWith('SET ') || elt.room.startsWith('Smith '))) {
            sections.push(elt);
            if (!rooms.includes(elt.room))
                rooms.push(elt.room);
        }
    }
    rooms.sort();
    let row_key = build_room_time_grid(rooms, [['M', 7*60, 18*60], ['T', 7*60, 18*60]]);

    let prefix_colors = {
        'CS':   '#f42f82',
        'IT':   '#1259B2',
        'SE':   '#8068c0',
        'BIOL': '#309090',
        'CHEM': '#ff9a3c',
        'ENVS': '#557b3f',
        'MECH': 'silver',
        'MATH': '#a03333',
        'GEOG': 'darkblue',
        'GEO':  'brown',
        'PHYS': 'purple',
    };

    const split_time = /^([MTWRFSU]+)(\d\d)(\d\d)\+(\d+)$/;
    let format_date = function (time_slot) {
        let parts = split_time.exec(time_slot);
        let start = Number(parts[2]) * 60 + Number(parts[3]);
        let end = start + Number(parts[4]);

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
        return `${start_hour}:${sm}${sam}&ndash;${end_hour}:${em} ${end_am}`;
    };

    let make_section = function (elt) {
        let box = document.createElement('div');
        box.classList.add('section');
        //box.style.backgroundColor = '#777777';
        //if (prefix_colors[section.prefixes[0]])
            //box.style.backgroundColor = prefix_colors[section.prefixes[0]];
        box.style.backgroundColor = hsv_to_rgb(hash(section.prefixes[0]));

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
        time.innerHTML = format_date(section.time_slot);

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

        return box;
    };
    let days_to_show = ['M', 'T'];
    for (section of sections) {
        for (day of days_to_show) {
            let parts = split_time.exec(section.time_slot);
            if (parts[1].indexOf(day) < 0) continue;

            let box = make_section(section);
            schedule.appendChild(box);

            box.style.gridColumn = rooms.indexOf(section.room) + 2;
            let start = (Number(parts[2])*60 + Number(parts[3]));
            let duration = Number(parts[4]);
            box.style.gridRow = '' + row_key[day + start] + ' / span ' + (duration / 5);
        }
    }
});;
