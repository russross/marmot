BEGIN;

DELETE FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placements (score, sort_score, optimum_score_prefix, comment, created_at, modified_at)
VALUES ('<10×1,13×1,18×1>', '<<89×01,86×01,81×01>>', '[0,0,0,0,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0]', 'Actual Fall 2026 Computing + Math schedule', '2026-04-14 12:30:45', '2026-04-14 12:30:45');

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 1030-01', 'MW1330+75', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 1400-01', 'TR1500+75', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 1400-02', 'MW1330+75', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 1400-03', 'TR1200+75', 'Smith 117'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 1400-04', 'TR1330+75', 'Smith 116'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 1410-01', 'MWF0900+50', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 1410-02', 'MWF1000+50', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 1500-01', 'TR0900+75', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 2100-01', 'MWF0900+50', 'Smith 116'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 2420-01', 'MW1500+75', 'Smith 116'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 2420-02', 'MWF1100+50', 'Smith 116'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 2450-01', 'MW1500+75', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 2450-02', 'TR1200+75', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 2500-01', 'MWF1000+50', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 2810-01', 'TR1500+75', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 2810-02', 'MW1500+75', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 3005-01', 'MW1330+75', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 3150-01', 'MWF1100+50', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 3400-01', 'TR1330+75', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 3500-01', 'MW1330+75', 'Smith 113'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 3520-01', 'MW1200+75', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 3530-01', 'TR1500+75', 'Smith 116'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 4300-01', 'MW1330+75', 'Smith 116'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 4410-01', 'TR1500+75', 'Smith 117'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 4420-01', 'MW1500+75', 'Smith 117'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 4480R-01', 'MW1200+75', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 4991R-01', 'F1400+50', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 4992R-01', 'F1300+50', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 4995-01', 'MWF1000+50', 'Smith 113'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'CS 6300-50', 'W1800+150', 'Smith 116'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 1100-01', 'TR0900+75', 'Smith 113'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 1100-02', 'TR1030+75', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 1100-03', 'MWF1100+50', 'Smith 113'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 1200-01', 'MW1500+75', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 2300-01', 'MWF1000+50', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 2400-01', 'TR1330+75', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 2700-01', 'TR1030+75', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 2750-01', 'TR0900+75', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 3100-01', 'MW1200+75', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 3300-01', 'TR0900+75', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 3510-01', 'MW1330+75', 'Smith 117'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 4200-01', 'TR1500+75', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 4400-01', 'TR1200+75', 'Smith 107'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'IT 4991R-01', 'W1630+100', 'Smith 108'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0900-01', 'MW1500+100', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0900-02', 'MTWR0900+50', 'Snow 147'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0900-03', 'TR1300+100', 'Snow 150'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0900-04', 'MW1630+100', 'Snow 151'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0900-05', 'MTWR1000+50', 'Snow 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0900-06', 'MW1500+100', 'Snow 147'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0900-50', 'MW1800+100', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0980-02', 'MTWR1000+50', 'Snow 147'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0980-04', 'MW1300+100', 'Snow 125'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0980-05', 'MTWR1100+50', 'Snow 150'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0980-06', 'MTWR0900+50', 'Snow 145'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0980-07', 'TR1300+100', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0980-08', 'MTWR1000+50', 'Snow 003'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0980-09', 'TR1630+100', 'Snow 150'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 0980-10', 'MTWR0800+50', 'Snow 147'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1010-01', 'MTWR0900+50', 'Snow 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1010-02', 'MTWR1200+50', 'Snow 145'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1010-03', 'MTWR1100+50', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1010-04', 'MW1500+100', 'Snow 124'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1010-06', 'MW1300+100', 'Snow 150'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1010-07', 'MTWR1000+50', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1010-08', 'MTWR1100+50', 'Snow 124'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1030-01', 'TR0730+75', 'Snow 151'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1030-02', 'MWF0900+50', 'Snow 124'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1030-03', 'TR1200+75', 'Snow 003'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1030-04', 'TR0900+75', 'Snow 125'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1030-05', 'TR1330+75', 'Snow 147'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1030-06', 'MWF1100+50', 'Snow 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-01', 'MWF0800+50', 'Snow 124'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-02', 'TR1500+75', 'Snow 003'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-03', 'MWF0900+50', 'Snow 150'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-04', 'TR1500+75', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-05', 'TR1030+75', 'Snow 145'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-06', 'TR1330+75', 'Snow 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-07', 'MW1200+75', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-08', 'MWF1100+50', 'Snow 125'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-14', 'MWF1100+50', 'Snow 151'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-15', 'TR1630+75', 'Snow 003'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-16', 'MW1630+75', 'Snow 145'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1040-17', 'MWF0900+50', 'Snow 125'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1050-01', 'MTWR0800+50', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1050-02', 'MTWR0900+50', 'Snow 151'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1050-03', 'MTWR1200+50', 'Snow 124'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1050-04', 'MTWR0900+50', 'Snow 003'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1050-05', 'MTWR1100+50', 'Snow 147'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1050-06', 'MW1300+100', 'Snow 124'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1050-07', 'MTWR1200+50', 'Snow 125'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1060-03', 'TR1030+75', 'Snow 151'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1060-04', 'MW1630+75', 'Snow 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1060-05', 'TR1200+75', 'Snow 151'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1080-01', 'MTWRF0800+50', 'Snow 125'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1210-01', 'TR1500+100', 'Snow 124'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1210-02', 'MTWR0900+50', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1210-03', 'MW1300+100', 'Snow 003'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1210-04', 'MTWR1200+50', 'Snow 150'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1220-01', 'MTWR0800+50', 'Snow 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 1220-02', 'MTWR1100+50', 'Snow 003'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 2010-01', 'MW1200+75', 'Snow 147'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 2010-02', 'T1630+150', 'Snow 144'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 2020-01', 'W1630+150', 'Snow 003'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 2050-01', 'MW1200+75', 'Snow 151'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 2210-01', 'MTWR0800+50', 'Snow 150'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 2270-01', 'TR1200+75', 'Snow 147'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 3120-01', 'TR1030+75', 'Snow 125'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 3400-01', 'TR0900+75', 'Snow 124'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 3410-01', 'W1200+50', 'Snow 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 3500-01', 'TR1200+75', 'Snow 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 3700-01', 'MW1630+75', 'Snow 150'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 4000-01', 'MWF1100+50', 'Snow 145'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'MATH 4500-01', 'R1630+150', 'Snow 145'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SA 1400-01', 'TR0930+80', NULL
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SA 1400-02', 'TR1200+80', NULL
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SD 6100-01', 'T1630+150', 'Smith 117'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SD 6110-01', 'M1630+150', 'Smith 117'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SD 6400-01', 'W1630+150', 'Smith 117'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SD 6450-01', 'R1630+150', 'Smith 117'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 1400-01', 'TR1330+75', 'Smith 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 1400-02', 'TR1330+75', 'Smith 113'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 3010-01', 'TR1200+75', 'Smith 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 3150-01', 'TR1330+75', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 3200-01', 'TR1030+75', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 3500-01', 'TR1200+75', 'Smith 109'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 3550-01', 'TR1500+75', 'Smith 113'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 4930R-01', 'TR1530+60', 'Smith 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 4990-01', 'R1630+150', 'Smith 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_sections (placement_id, section, time_slot, room)
SELECT placement_id, 'SE 4990-02', 'MW1200+75', 'Smith 112'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_penalties (placement_id, priority, message)
SELECT placement_id, 18, 'Russ Ross is scheduled to teach CS 3520-01 at MW1200+75'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_penalty_sections (placement_penalty_id, section)
SELECT pp.placement_penalty_id, 'CS 3520-01'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 18 AND pp.message = 'Russ Ross is scheduled to teach CS 3520-01 at MW1200+75';

INSERT INTO placement_penalty_faculty (placement_penalty_id, faculty)
SELECT pp.placement_penalty_id, 'Russ Ross'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 18 AND pp.message = 'Russ Ross is scheduled to teach CS 3520-01 at MW1200+75';

INSERT INTO placement_penalties (placement_id, priority, message)
SELECT placement_id, 10, 'Matthew S Smith has a run of back-to-back classes that lasts 3h10m'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_penalty_sections (placement_penalty_id, section)
SELECT pp.placement_penalty_id, 'MATH 0900-50'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 10 AND pp.message = 'Matthew S Smith has a run of back-to-back classes that lasts 3h10m';

INSERT INTO placement_penalty_sections (placement_penalty_id, section)
SELECT pp.placement_penalty_id, 'MATH 1060-04'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 10 AND pp.message = 'Matthew S Smith has a run of back-to-back classes that lasts 3h10m';

INSERT INTO placement_penalty_faculty (placement_penalty_id, faculty)
SELECT pp.placement_penalty_id, 'Matthew S Smith'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 10 AND pp.message = 'Matthew S Smith has a run of back-to-back classes that lasts 3h10m';

INSERT INTO placement_penalties (placement_id, priority, message)
SELECT placement_id, 13, 'McKay Sullivan has to wait 2h10m between clusters of classes'
FROM placements WHERE comment = 'Actual Fall 2026 Computing + Math schedule';

INSERT INTO placement_penalty_sections (placement_penalty_id, section)
SELECT pp.placement_penalty_id, 'MATH 1060-03'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 13 AND pp.message = 'McKay Sullivan has to wait 2h10m between clusters of classes';

INSERT INTO placement_penalty_sections (placement_penalty_id, section)
SELECT pp.placement_penalty_id, 'MATH 1210-04'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 13 AND pp.message = 'McKay Sullivan has to wait 2h10m between clusters of classes';

INSERT INTO placement_penalty_sections (placement_penalty_id, section)
SELECT pp.placement_penalty_id, 'MATH 2210-01'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 13 AND pp.message = 'McKay Sullivan has to wait 2h10m between clusters of classes';

INSERT INTO placement_penalty_sections (placement_penalty_id, section)
SELECT pp.placement_penalty_id, 'MATH 4000-01'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 13 AND pp.message = 'McKay Sullivan has to wait 2h10m between clusters of classes';

INSERT INTO placement_penalty_faculty (placement_penalty_id, faculty)
SELECT pp.placement_penalty_id, 'McKay Sullivan'
FROM placement_penalties pp
JOIN placements p ON p.placement_id = pp.placement_id
WHERE p.comment = 'Actual Fall 2026 Computing + Math schedule' AND pp.priority = 13 AND pp.message = 'McKay Sullivan has to wait 2h10m between clusters of classes';

COMMIT;
