#!/usr/bin/env python3

import queries

DB_FILE = 'timetable.db'
db = queries.DB(DB_FILE)

print('building cset conflicts')

db.make_program('Envs Envs emphasis', 'Earth')
db.make_conflict('Envs Envs emphasis', 'core requirements', 99, 'maximize',
    ['ENVS 1210', 'ENVS 1215',
    'ENVS 2210',
    'GEO 1110', 'GEO 1115',
    'GEOG 3600', 'GEOG 3605',
    'CHEM 1210', 'CHEM 1215',
    'CHEM 1220', 'CHEM 1225',
    'BIOL 1610', 'BIOL 1615',
    'MATH 1060',
    'ENVS 2700R',
    'ENVS 4910',
    'ENVS 3920'])

db.make_program('Envs Geo emphasis', 'Earth')
db.make_conflict('Envs Geo emphasis', 'core requirements', 99, 'maximize',
    ['ENVS 1210', 'ENVS 1215',
    'ENVS 2210',
    'GEO 1110', 'GEO 1115',
    'GEOG 3600', 'GEOG 3605',
    'CHEM 1210', 'CHEM 1215',
    'CHEM 1220', 'CHEM 1225',
    'BIOL 1610', 'BIOL 1615',
    'MATH 1060',
    'GEO 1220', 'GEO 1225',
    'GEO 2700R'])

db.make_program('Geological Sciences', 'Earth')
db.make_conflict('Geological Sciences', 'core requirements', 99, 'maximize',
    ['BIOL 3110',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'GEO 1110', 'GEO 1115', 'GEO 1220', 'GEO 1225', 'GEO 2700R', 'GEO 2990R',
    'GEO 3060', 'GEO 3180', 'GEO 3200', 'GEO 3500', 'GEO 3550',
    'GEO 3600', 'GEO 3700', 'GEO 3710', 'GEO 4600', 'GEO 4800R',
    'GEOG 3600', 'GEOG 3605',
    'MATH 1210',
    'PHYS 2010', 'PHYS 2015', 'PHYS 2210', 'PHYS 2215',
    'PHYS 2020', 'PHYS 2025', 'PHYS 2220', 'PHYS 2225',
    'GEO 3000', 'GEO 3910',
    'ENVS 3910', 'ENVS 3920', 'ENVS 3930',
    'GEOG 3930'])
db.make_conflict('Geological Sciences', 'take one set', 0, 'minimize',
    ['PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225'])

db.make_program('Bioinformatics', 'Biology')
db.make_conflict('Bioinformatics', 'core requirements', 99, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625',
    'BIOL 3010', 'BIOL 3300', 'BIOL 3030', 'BIOL 4010', 'BIOL 4300',
    'BIOL 4305', 'BIOL 4310', 'BIOL 4320', 'BIOL 4810R', 'BIOL 4910',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 3310',
    'IT 1100', 'IT 2300',
    'MATH 1210', 'MATH 3060'])
db.make_conflict('Bioinformatics', 'pick one tech lab course', 30, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625',
    'BIOL 3010', 'BIOL 3300', 'BIOL 3030', 'BIOL 4010', 'BIOL 4300',
    'BIOL 4305', 'BIOL 4310', 'BIOL 4320', 'BIOL 4810R', 'BIOL 4910',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 3310',
    'IT 1100', 'IT 2300',
    'MATH 1210', 'MATH 3060',
    'BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050', 'BIOL 2300'])
db.make_conflict('Bioinformatics', 'take one technical lab', 0, 'minimize',
    ['BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050', 'BIOL 2300'])

db.make_program('Biology Education emphasis', 'Biology')
db.make_conflict('Biology Education emphasis', 'core requirements', 99, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'BIOL 3010', 'BIOL 3030',
    'MATH 1050',
    'BIOL 2320', 'BIOL 2325', 'BIOL 3140', 'BIOL 3145', 'BIOL 2420', 'BIOL 2425', 'BIOL 4500', 'BIOL 4505',
    'BIOL 3040', 'BIOL 3045', 'BIOL 2060', 'BIOL 2065', 'BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555',
    'BIOL 2400', 'BIOL 2405', 'BIOL 3200', 'BIOL 3205', 'BIOL 4260', 'BIOL 4265', 'BIOL 4270',
    'BIOL 4275', 'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440',
    'SCI 2600', 'SCI 4700'])
db.make_conflict('Biology Education emphasis', 'take one pair', 0, 'minimize',
    ['BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555'])
db.make_conflict('Biology Education emphasis', 'yet another take a pair', 0, 'minimize',
    ['BIOL 3200', 'BIOL 3205', 'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275',
    'BIOL 4280', 'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440'])
db.make_conflict('Biology Education emphasis', 'only need one set', 0, 'minimize',
    ['BIOL 3200', 'BIOL 3205', 'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275',
    'BIOL 4280', 'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440'])
db.make_conflict('Biology Education emphasis', 'choose one set', 0, 'minimize',
    ['BIOL 2320', 'BIOL 2325', 'BIOL 3140', 'BIOL 3145'])
db.make_conflict('Biology Education emphasis', 'another choice', 0, 'minimize',
    ['BIOL 2420', 'BIOL 2425', 'BIOL 4500', 'BIOL 4505'])
db.make_conflict('Biology Education emphasis', 'final choice', 0, 'minimize',
    ['BIOL 2060', 'BIOL 2065', 'BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555'])

db.make_program('Biology Biology emphasis', 'Biology')
db.make_conflict('Biology Biology emphasis', 'core requirements', 99, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'BIOL 3010', 'BIOL 3030',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225', 'CHEM 2310', 'CHEM 2315', 'CHEM 2320', 'CHEM 2325',
    'MATH 1210',
    'BIOL 3040', 'BIOL 3045', 'BIOL 3155',
    'MATH 3060',
    'BIOL 4910'])
db.make_conflict('Biology Biology emphasis', 'choices', 45, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'BIOL 3010', 'BIOL 3030',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225', 'CHEM 2310', 'CHEM 2315', 'CHEM 2320', 'CHEM 2325',
    'MATH 1210',
    'BIOL 3040', 'BIOL 3045', 'BIOL 3155',
    'MATH 3060',
    'BIOL 4910',
    'PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025', 'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    'BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050', 'BIOL 2300',
    'BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555',
    'BIOL 3420', 'BIOL 4500', 'BIOL 4505', 'BIOL 4600', 'BIOL 4605',
    'BIOL 3200', 'BIOL 3205', 'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275',
    'BIOL 4280', 'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440'])
db.make_conflict('Biology Biology emphasis', 'electives', 30, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'BIOL 3010', 'BIOL 3030',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225', 'CHEM 2310', 'CHEM 2315', 'CHEM 2320', 'CHEM 2325',
    'MATH 1210',
    'BIOL 3040', 'BIOL 3045', 'BIOL 3155',
    'MATH 3060',
    'BIOL 4910',
    'PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025', 'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    'BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050', 'BIOL 2300',
    'BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555',
    'BIOL 3420', 'BIOL 4500', 'BIOL 4505', 'BIOL 4600', 'BIOL 4605',
    'BIOL 3200', 'BIOL 3205', 'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275',
    'BIOL 4280', 'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440',
    'BTEC 3020', 'CHEM 3510', 'CHEM 3515', 'CHEM 3520', 'CHEM 3525',
    'BTEC 3010', 'BTEC 3040', 'BTEC 3050', 'BTEC 4020', 'BTEC 4040', 'BTEC 4050', 'BTEC 4060'])
db.make_conflict('Biology Biology emphasis', 'take one set', 0, 'minimize',
    ['PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225'])
db.make_conflict('Biology Biology emphasis', 'take one technical lab', 0, 'minimize',
    ['BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050', 'BIOL 2300'])
db.make_conflict('Biology Biology emphasis', 'take one pair', 0, 'minimize',
    ['BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555'])
db.make_conflict('Biology Biology emphasis', 'another take one pair', 0, 'minimize',
    ['BIOL 3420', 'BIOL 4500', 'BIOL 4505', 'BIOL 4600', 'BIOL 4605'])
db.make_conflict('Biology Biology emphasis', 'yet another take a pair', 0, 'minimize',
    ['BIOL 3200', 'BIOL 3205', 'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275',
    'BIOL 4280', 'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440'])
db.make_conflict('Biology Biology emphasis', 'only need one set', 0, 'minimize',
    ['BIOL 3200', 'BIOL 3205', 'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275',
    'BIOL 4280', 'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440'])

db.make_program('Biology Biomed emphasis', 'Biology')
db.make_conflict('Biology Biomed emphasis', 'core requirements', 99, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'BIOL 3010', 'BIOL 3030', 'BIOL 3040',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225', 'CHEM 2310', 'CHEM 2315',
    'CHEM 2320', 'CHEM 2325', 'CHEM 3510', 'CHEM 3515',
    'PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    'BIOL 2320', 'BIOL 2325', 'BIOL 3420',
    'MATH 3060',
    'BIOL 3155', 'BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555', 'BIOL 4910',
    'BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050',
    'BIOL 2300'])
db.make_conflict('Biology Biomed emphasis', 'electives', 30, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'BIOL 3010', 'BIOL 3030', 'BIOL 3040',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225', 'CHEM 2310', 'CHEM 2315',
    'CHEM 2320', 'CHEM 2325', 'CHEM 3510', 'CHEM 3515',
    'PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    'BIOL 2320', 'BIOL 2325', 'BIOL 3420',
    'MATH 3060',
    'BIOL 3155', 'BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555', 'BIOL 4910',
    'BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050',
    'BIOL 2300',
    'BIOL 3000R', 'BIOL 3100', 'BIOL 3110', 'BIOL 3120', 'BIOL 3140', 'BIOL 3145',
    'BIOL 3230R', 'BIOL 3250', 'BIOL 3360', 'BIOL 3460', 'BIOL 3470',
    'BIOL 4300', 'BIOL 4305', 'BIOL 4440', 'BIOL 4930R',
    'CHEM 3520', 'CHEM 3525',
    'MATH 1210'])
db.make_conflict('Biology Biomed emphasis', 'take one set', 0, 'minimize',
    ['PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225'])
db.make_conflict('Biology Biomed emphasis', 'take one technical lab', 0, 'minimize',
    ['BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050', 'BIOL 2300'])
db.make_conflict('Biology Biomed emphasis', 'take one pair', 0, 'minimize',
    ['BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555'])

db.make_program('Biology Natural Sciences emphasis', 'Biology')
db.make_conflict('Biology Natural Sciences emphasis', 'core requirements', 99, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'BIOL 2400', 'BIOL 2405',
    'BIOL 3010', 'BIOL 3030', 'BIOL 3040', 'BIOL 3045', 'BIOL 3110', 'BIOL 3120', 'BIOL 4910',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'ENVS 1210', 'ENVS 1215',
    'GEO 1110', 'GEO 1115',
    'GEOG 3600', 'GEOG 3605',
    'MATH 1040', 'MATH 1050',
    'PHYS 1010', 'PHYS 1015', 'PHYS 2010', 'PHYS 2015'])
db.make_conflict('Biology Natural Sciences emphasis', 'choices', 45, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'BIOL 2400', 'BIOL 2405',
    'BIOL 3010', 'BIOL 3030', 'BIOL 3040', 'BIOL 3045', 'BIOL 3110', 'BIOL 3120', 'BIOL 4910',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'ENVS 1210', 'ENVS 1215',
    'GEO 1110', 'GEO 1115',
    'GEOG 3600', 'GEOG 3605',
    'MATH 1040', 'MATH 1050',
    'PHYS 1010', 'PHYS 1015', 'PHYS 2010', 'PHYS 2015',
    'BIOL 3200', 'BIOL 3340', 'BIOL 3345', 'BIOL 4200', 'BIOL 4205', 'BIOL 4260',
    'BIOL 4265', 'BIOL 4270', 'BIOL 4275', 'BIOL 4280', 'BIOL 4350', 'BIOL 4355',
    'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440', 'BIOL 4600', 'BIOL 4605'])
db.make_conflict('Biology Natural Sciences emphasis', 'electives', 30, 'maximize',
    ['BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'BIOL 2400', 'BIOL 2405',
    'BIOL 3010', 'BIOL 3030', 'BIOL 3040', 'BIOL 3045', 'BIOL 3110', 'BIOL 3120', 'BIOL 4910',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'ENVS 1210', 'ENVS 1215',
    'GEO 1110', 'GEO 1115',
    'GEOG 3600', 'GEOG 3605',
    'MATH 1040', 'MATH 1050',
    'PHYS 1010', 'PHYS 1015', 'PHYS 2010', 'PHYS 2015',
    'BIOL 3200', 'BIOL 3340', 'BIOL 3345', 'BIOL 4200', 'BIOL 4205', 'BIOL 4260',
    'BIOL 4265', 'BIOL 4270', 'BIOL 4275', 'BIOL 4280', 'BIOL 4350', 'BIOL 4355',
    'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440', 'BIOL 4600', 'BIOL 4605',
    'BIOL 3100', 'BIOL 3140', 'BIOL 3145', 'BIOL 3250', 'BIOL 3360', 'BIOL 3450', 'BIOL 3455',
    'BIOL 3550', 'BIOL 3555', 'BIOL 4300', 'BIOL 4305', 'BIOL 4500', 'BIOL 4505',
    'BIOL 4810R', 'BIOL 4930R',
    'GEOG 4140', 'GEOG 4180',
    'MATH 1210', 'MATH 3060',
    'BIOL 3155'])
db.make_conflict('Biology Natural Sciences emphasis', 'take one pair', 0, 'minimize',
    ['BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555'])
db.make_conflict('Biology Natural Sciences emphasis', 'only need one set', 0, 'minimize',
    ['BIOL 3200', 'BIOL 3205', 'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275',
    'BIOL 4280', 'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440'])

db.make_program('Biology Integrated Education Sciences', 'Biology')
db.make_conflict('Biology Integrated Education Sciences', 'core requirements', 99, 'maximize',
    ['CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'PHYS 2010', 'PHYS 2015',
    'MATH 1050', 'MATH 1060', 'MATH 1080',
    'BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625', 'BIOL 2320', 'BIOL 2325',
    'BIOL 3140', 'BIOL 3145', 'BIOL 2420', 'BIOL 2425', 'BIOL 4500', 'BIOL 4505',
    'BIOL 3010', 'BIOL 3030', 'BIOL 3040', 'BIOL 3045', 'BIOL 2060', 'BIOL 2065',
    'BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555',
    'BIOL 2400', 'BIOL 2405', 'BIOL 3200', 'BIOL 3205',
    'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275', 'BIOL 4350', 'BIOL 4355',
    'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440',
    'GEO 1110', 'GEO 1115',
    'PHYS 1040', 'PHYS 1045',
    'SCI 2600',
    'SCI 4700'])
db.make_conflict('Biology Integrated Education Sciences', 'take one pair', 0, 'minimize',
    ['BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555'])
db.make_conflict('Biology Integrated Education Sciences', 'only need one set', 0, 'minimize',
    ['BIOL 3200', 'BIOL 3205', 'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275',
    'BIOL 4280', 'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411', 'BIOL 4415', 'BIOL 4440'])
db.make_conflict('Biology Integrated Education Sciences', 'math 1050 or 1080', 0, 'minimize',
    ['MATH 1050', 'MATH 1080'])
db.make_conflict('Biology Integrated Education Sciences', 'math 1060 or 1080', 0, 'minimize',
    ['MATH 1060', 'MATH 1080'])
db.make_conflict('Biology Integrated Education Sciences', 'choose one set', 0, 'minimize',
    ['BIOL 2320', 'BIOL 2325', 'BIOL 3140', 'BIOL 3145'])
db.make_conflict('Biology Integrated Education Sciences', 'another choice', 0, 'minimize',
    ['BIOL 2420', 'BIOL 2425', 'BIOL 4500', 'BIOL 4505'])
db.make_conflict('Biology Integrated Education Sciences', 'final choice', 0, 'minimize',
    ['BIOL 2060', 'BIOL 2065', 'BIOL 3450', 'BIOL 3455', 'BIOL 3550', 'BIOL 3555'])

db.make_program('Chemistry Chemistry emphasis', 'Chemistry')
db.make_conflict('Chemistry Chemistry emphasis', 'core requirements', 99, 'maximize',
    ['MATH 1210', 'MATH 1220',
    'BIOL 1610', 'BIOL 1615',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'CHEM 2310', 'CHEM 2315', 'CHEM 2320', 'CHEM 2325', 'CHEM 2600', 'CHEM 2990R',
    'CHEM 3000', 'CHEM 3005', 'CHEM 3060', 'CHEM 3065', 'CHEM 3070', 'CHEM 3075',
    'CHEM 3100', 'CHEM 3300', 'CHEM 3510', 'CHEM 3515', 'CHEM 3520', 'CHEM 3525',
    'CHEM 4100', 'CHEM 4800R', 'CHEM 4910', 'CHEM 4200', 'CHEM 4310', 'CHEM 4510', 'CHEM 4610'])
db.make_conflict('Chemistry Chemistry emphasis', 'electives', 30, 'maximize',
    ['MATH 1210', 'MATH 1220',
    'BIOL 1610', 'BIOL 1615',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    'MATH 2210', 'MATH 2250', 'MATH 2270', 'MATH 2280', 'MATH 3060',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    'CHEM 2310', 'CHEM 2315', 'CHEM 2320', 'CHEM 2325', 'CHEM 2600', 'CHEM 2990R',
    'CHEM 3000', 'CHEM 3005', 'CHEM 3060', 'CHEM 3065', 'CHEM 3070', 'CHEM 3075',
    'CHEM 3100', 'CHEM 3300', 'CHEM 3510', 'CHEM 3515', 'CHEM 3520', 'CHEM 3525',
    'CHEM 4100', 'CHEM 4800R', 'CHEM 4910', 'CHEM 4200', 'CHEM 4310', 'CHEM 4510', 'CHEM 4610'])
db.make_conflict('Chemistry Chemistry emphasis', 'pair or single', 0, 'minimize',
    ['CHEM 2310', 'CHEM 2315', 'CHEM 3000'])
db.make_conflict('Chemistry Chemistry emphasis', 'math 2250 or 2270', 0, 'minimize',
    ['MATH 2270', 'MATH 2250'])
db.make_conflict('Chemistry Chemistry emphasis', 'math 2250 or 2280', 0, 'minimize',
    ['MATH 2280', 'MATH 2250'])

db.make_program('Chemistry Molecular Biology emphasis', 'Chemistry')
db.make_conflict('Chemistry Molecular Biology emphasis', 'core requirements', 99, 'maximize',
    ['CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225', 'CHEM 2310', 'CHEM 2315',
    'CHEM 2320', 'CHEM 2325', 'CHEM 2600', 'CHEM 2990R',
    'CHEM 3000', 'CHEM 3005', 'CHEM 3060', 'CHEM 3065', 'CHEM 3070', 'CHEM 3075', 'CHEM 3100',
    'CHEM 3300', 'CHEM 3510', 'CHEM 3515', 'CHEM 3520', 'CHEM 3525', 'CHEM 4910',
    'BIOL 1610', 'BIOL 1615', 'BIOL 3030', 'BIOL 3550', 'BIOL 3555', 'BIOL 4300', 'BIOL 4305',
    'MATH 1210', 'MATH 1220',
    'PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    'CHEM 4800R',
    'BIOL 4810R', 'BIOL 4890R'])
db.make_conflict('Chemistry Molecular Biology emphasis', 'electives', 30, 'maximize',
    ['CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225', 'CHEM 2310', 'CHEM 2315',
    'CHEM 2320', 'CHEM 2325', 'CHEM 2600', 'CHEM 2990R',
    'CHEM 3000', 'CHEM 3005', 'CHEM 3060', 'CHEM 3065', 'CHEM 3070', 'CHEM 3075', 'CHEM 3100',
    'CHEM 3300', 'CHEM 3510', 'CHEM 3515', 'CHEM 3520', 'CHEM 3525', 'CHEM 4910',
    'BIOL 1610', 'BIOL 1615', 'BIOL 3030', 'BIOL 3550', 'BIOL 3555', 'BIOL 4300', 'BIOL 4305',
    'MATH 1210', 'MATH 1220',
    'PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    'CHEM 4800R',
    'BIOL 4810R', 'BIOL 4890R',
    'CHEM 4100', 'CHEM 4610',
    'BIOL 3010', 'BIOL 3250', 'BIOL 3360', 'BIOL 3420',
    'BIOL 3450', 'BIOL 3455', 'BIOL 3470', 'BIOL 3460', 'BIOL 4400'])
db.make_conflict('Chemistry Molecular Biology emphasis', 'take one set', 0, 'minimize',
    ['PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225'])
db.make_conflict('Chemistry Molecular Biology emphasis', 'pair or single', 0, 'minimize',
    ['CHEM 2310', 'CHEM 2315', 'CHEM 3000'])

db.make_program('Chemistry Physical Sciences emphasis', 'Chemistry')
db.make_conflict('Chemistry Physical Sciences emphasis', 'core requirements', 99, 'maximize',
    ['SCI 4700',
    'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225', 'CHEM 2310', 'CHEM 2315', 'CHEM 3000',
    'GEO 1110', 'GEO 1115', 'GEO 1220', 'GEO 1225', 'GEO 3060',
    'PHYS 1040', 'PHYS 1045', 'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225', 'PHYS 3710',
    'BIOL 1610', 'BIOL 1615',
    'MATH 1210', 'MATH 1220',
    'SCI 2600', 'SCI 4800R',
    'CHEM 3510',
    'PHYS 3400'])
db.make_conflict('Chemistry Physical Sciences emphasis', 'pair or single', 0, 'minimize',
    ['CHEM 2310', 'CHEM 2315', 'CHEM 3000'])
db.make_conflict('Chemistry Physical Sciences emphasis', 'chem or phys', 0, 'minimize',
    ['CHEM 3510', 'PHYS 3400'])

db.make_program('Math', 'Math')
db.make_conflict('Math', 'core requirements', 99, 'maximize',
    ['MATH 1210', 'MATH 1220', 'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
    'MATH 3200', 'MATH 3400', 'MATH 3900', 'MATH 4000', 'MATH 4900',
    'CS 1400',
    'PHYS 2210', 'PHYS 2215'])
db.make_conflict('Math', 'electives', 30, 'maximize',
    ['MATH 1210', 'MATH 1220', 'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
    'MATH 3200', 'MATH 3400', 'MATH 3900', 'MATH 4000', 'MATH 4900',
    'CS 1400',
    'PHYS 2210', 'PHYS 2215',
    'MATH 3000', 'MATH 3100', 'MATH 3150', 'MATH 3210', 'MATH 3450',
    'MATH 3500', 'MATH 3605', 'MATH 3700',
    'MATH 4010', 'MATH 4100', 'MATH 4200', 'MATH 4250', 'MATH 4550',
    'MATH 4800', 'MATH 4890R',
    'PHYS 2220', 'PHYS 2225'])

db.make_program('Math ACM Data Analytics', 'Math')
db.make_conflict('Math ACM Data Analytics', 'core requirements', 99, 'maximize',
    ['CS 1400', 'CS 1410',
    'MATH 1210', 'MATH 1220', 'MATH 2200', 'CS 3310',
    'MATH 2210', 'MATH 2270', 'MATH 2280', 'MATH 3400', 'MATH 3700',
    'MATH 4250', 'MATH 4800', 'MATH 4890R', 'MATH 4900',
    'MATH 2050', 'MATH 3050', 'MATH 3450',
    'IT 1100', 'IT 2300', 'IT 2400', 'IT 4310'])
db.make_conflict('Math ACM Data Analytics', 'electives', 30, 'maximize',
    ['CS 1400', 'CS 1410',
    'MATH 1210', 'MATH 1220', 'MATH 2200', 'CS 3310',
    'MATH 2210', 'MATH 2270', 'MATH 2280', 'MATH 3400', 'MATH 3700',
    'MATH 4250', 'MATH 4800', 'MATH 4890R', 'MATH 4900',
    'MATH 2050', 'MATH 3050', 'MATH 3450',
    'IT 1100', 'IT 2300', 'IT 2400', 'IT 4310',
    'CS 3005', 'IT 4510',
    'MATH 3100', 'MATH 3150', 'MATH 3120', 'MATH 3200', 'MATH 3500',
    'MATH 3900', 'MATH 3905', 'MATH 4000', 'MATH 4005', 'MATH 4010',
    'MATH 4100', 'MATH 4200', 'MATH 4330', 'MATH 4550'])
db.make_conflict('Math ACM Data Analytics', 'discrete math', 0, 'minimize',
    ['MATH 2200', 'CS 3310'])

db.make_program('Math Education', 'Math')
db.make_conflict('Math Education', 'core requirements', 99, 'maximize',
    ['MATH 1040', 'MATH 1210', 'MATH 1220', 'MATH 2200', 'MATH 2210',
    'MATH 2270', 'MATH 2280', 'MATH 3000', 'MATH 3010', 'MATH 3020',
    'MATH 3100', 'MATH 3120', 'MATH 3200', 'MATH 3400', 'MATH 4000',
    'CS 1400',
    'PHYS 2210', 'PHYS 2215'])

db.make_program('Math ACM Computational Mathematics', 'Math')
db.make_conflict('Math ACM Computational Mathematics', 'core requirements', 99, 'maximize',
    ['CS 1400', 'CS 1410',
    'MATH 1210', 'MATH 1220', 'MATH 2200', 'CS 3310',
    'MATH 2210', 'MATH 2270', 'MATH 2280', 'MATH 3400', 'MATH 3700',
    'MATH 4250', 'MATH 4800', 'MATH 4890R', 'MATH 4900',
    'CS 2420', 'CS 3005',
    'MATH 2050', 'MATH 3150', 'MATH 3500', 'MATH 4550',
    'MECH 2010', 'MECH 2030',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225'])
db.make_conflict('Math ACM Computational Mathematics', 'electives', 30, 'maximize',
    ['CS 1400', 'CS 1410',
    'MATH 1210', 'MATH 1220', 'MATH 2200', 'CS 3310',
    'MATH 2210', 'MATH 2270', 'MATH 2280', 'MATH 3400', 'MATH 3700',
    'MATH 4250', 'MATH 4800', 'MATH 4890R', 'MATH 4900',
    'CS 2420', 'CS 3005',
    'MATH 2050', 'MATH 3150', 'MATH 3500', 'MATH 4550',
    'MECH 2010', 'MECH 2030',
    'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    'MATH 3050', 'MATH 3450', 'MATH 3120', 'MATH 3100', 'MATH 3900', 'MATH 3905',
    'MATH 4000', 'MATH 4005', 'MATH 4010', 'MATH 4100', 'MATH 4330',
    'MATH 3200', 'MATH 4200',
    'MECH 3600', 'MECH 3700', 'MECH 3705',
    'PHYS 3400', 'PHYS 3710'])
db.make_conflict('Math ACM Computational Mathematics', 'discrete math', 0, 'minimize',
    ['MATH 2200', 'CS 3310'])

db.make_program('Math ACM Actuarial Sciences', 'Math')
db.make_conflict('Math ACM Actuarial Sciences', 'core requirements', 99, 'maximize',
    ['CS 1400', 'CS 1410',
    'MATH 1210', 'MATH 1220', 'MATH 2200', 'CS 3310',
    'MATH 2210', 'MATH 2270', 'MATH 2280', 'MATH 3400', 'MATH 3700',
    'MATH 4250', 'MATH 4800', 'MATH 4890R', 'MATH 4900',
    'CS 2420',
    'MATH 3410', 'MATH 3450', 'MATH 4400', 'MATH 4410'])
db.make_conflict('Math ACM Actuarial Sciences', 'electives', 30, 'maximize',
    ['CS 1400', 'CS 1410',
    'MATH 1210', 'MATH 1220', 'MATH 2200', 'CS 3310',
    'MATH 2210', 'MATH 2270', 'MATH 2280', 'MATH 3400', 'MATH 3700',
    'MATH 4250', 'MATH 4800', 'MATH 4890R', 'MATH 4900',
    'CS 2420',
    'MATH 3410', 'MATH 3450', 'MATH 4400', 'MATH 4410',
    'MATH 3050', 'MATH 3120', 'MATH 3150', 'MATH 3200', 'MATH 3100',
    'MATH 3500', 'MATH 3900', 'MATH 3905', 'MATH 4000', 'MATH 4005',
    'MATH 4200', 'MATH 4010', 'MATH 4100', 'MATH 4330', 'MATH 4550'])
db.make_conflict('Math ACM Actuarial Sciences', 'discrete math', 0, 'minimize',
    ['MATH 2200', 'CS 3310'])

print('building cset buildings and rooms')
db.make_building('Brown')
db.make_room('Brown 201', 65, [])

db.make_building('COE')
db.make_room('COE 121', 50, [])

db.make_building('HCC')
db.make_room('HCC 476', 20, [])

db.make_building('SET')
db.make_room('SET 101', 18, [])
db.make_room('SET 102', 18, [])
db.make_room('SET 104', 40, [])
db.make_room('SET 105', 60, ['Science medium lecture', 'Science small lecture'])
db.make_room('SET 106', 125, ['Science large lecture', 'Science medium lecture', 'Science small lecture'])
db.make_room('SET 201', 65, ['Science medium lecture', 'Science small lecture'])
db.make_room('SET 213', 20, [])
db.make_room('SET 214', 20, [])
db.make_room('SET 215', 20, [])
db.make_room('SET 216', 24, [])
db.make_room('SET 219', 24, [])
db.make_room('SET 225', 20, [])
db.make_room('SET 226', 40, [])
db.make_room('SET 301', 65, ['Science medium lecture', 'Science small lecture'])
db.make_room('SET 303', 12, [])
db.make_room('SET 304', 18, [])
db.make_room('SET 308', 24, [])
db.make_room('SET 309', 20, [])
db.make_room('SET 310', 14, [])
db.make_room('SET 312', 20, [])
db.make_room('SET 318', 24, [])
db.make_room('SET 319', 24, [])
db.make_room('SET 404', 16, [])
db.make_room('SET 405', 24, [])
db.make_room('SET 407', 24, [])
db.make_room('SET 408', 15, [])
db.make_room('SET 409', 24, [])
db.make_room('SET 410', 24, [])
db.make_room('SET 412', 24, [])
db.make_room('SET 418', 48, ['Science small lecture'])
db.make_room('SET 420', 48, ['Science small lecture'])
db.make_room('SET 501', 20, [])
db.make_room('SET 522', 24, [])
db.make_room('SET 523', 24, [])
db.make_room('SET 524', 45, ['Science small lecture'])
db.make_room('SET 526', 24, [])
db.make_room('SET 527', 24, [])

db.make_building('Snow')
db.make_room('Snow 103', 16, [])
db.make_room('Snow 112', 42, ['Math lecture'])
db.make_room('Snow 113', 36, [])
db.make_room('Snow 124', 42, ['Math lecture'])
db.make_room('Snow 125', 42, ['Math lecture'])
db.make_room('Snow 128', 40, ['Science small lecture', 'Science Snow lecture'])
db.make_room('Snow 144', 42, ['Math lecture'])
db.make_room('Snow 145', 42, ['Math lecture'])
db.make_room('Snow 147', 42, ['Math lecture'])
db.make_room('Snow 150', 42, ['Math lecture'])
db.make_room('Snow 151', 42, ['Math lecture'])
db.make_room('Snow 204', 10, [])
db.make_room('Snow 208', 24, ['Science small lecture', 'Science Snow lecture'])
db.make_room('Snow 216', 45, ['Science small lecture', 'Science Snow lecture'])
db.make_room('Snow 3', 42, ['Math lecture'])


print('building cset time slots')
db.make_time_slot('F0800+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('F0900+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('F1000+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('F1100+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('F1200+50', ['1 credit extended bell schedule'])
db.make_time_slot('M0800+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('M0900+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('M1000+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('M1100+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('M1200+50', ['1 credit extended bell schedule'])
db.make_time_slot('R0800+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('R0900+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('R1000+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('R1030+50', ['1 credit extended bell schedule'])
db.make_time_slot('R1100+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('R1200+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('R1300+50', ['1 credit extended bell schedule'])
db.make_time_slot('R1400+50', ['1 credit extended bell schedule'])
db.make_time_slot('R1500+50', ['1 credit extended bell schedule'])
db.make_time_slot('R1600+50', ['1 credit extended bell schedule'])
db.make_time_slot('R1800+50', ['1 credit evening'])
db.make_time_slot('T0800+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('T0900+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('T1000+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('T1030+50', ['1 credit extended bell schedule'])
db.make_time_slot('T1100+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('T1200+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('T1300+50', ['1 credit extended bell schedule'])
db.make_time_slot('T1400+50', ['1 credit extended bell schedule'])
db.make_time_slot('T1500+50', ['1 credit extended bell schedule'])
db.make_time_slot('T1600+50', ['1 credit extended bell schedule'])
db.make_time_slot('T1800+50', ['1 credit evening'])
db.make_time_slot('W0800+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('W0900+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('W1000+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('W1100+50', ['1 credit bell schedule', '1 credit extended bell schedule'])
db.make_time_slot('W1200+50', ['1 credit extended bell schedule'])
db.make_time_slot('W1800+50', ['1 credit evening'])
db.make_time_slot('MF0800+50', ['2 credit lecture'])
db.make_time_slot('MF0900+50', ['2 credit lecture'])
db.make_time_slot('MF1000+50', ['2 credit lecture'])
db.make_time_slot('MF1100+50', ['2 credit lecture'])
db.make_time_slot('MW0730+50', ['2 credit lecture'])
db.make_time_slot('MW0800+50', ['2 credit lecture'])
db.make_time_slot('MW0900+50', ['2 credit lecture'])
db.make_time_slot('MW1000+50', ['2 credit lecture'])
db.make_time_slot('MW1100+50', ['2 credit lecture'])
db.make_time_slot('MW1200+50', ['2 credit lecture'])
db.make_time_slot('MW1330+50', ['2 credit lecture'])
db.make_time_slot('MW1500+50', ['2 credit lecture'])
db.make_time_slot('MW1630+50', ['2 credit lecture'])
db.make_time_slot('TR0730+50', ['2 credit lecture'])
db.make_time_slot('TR0900+50', ['2 credit lecture'])
db.make_time_slot('TR1000+50', [])
db.make_time_slot('TR1030+50', ['2 credit lecture'])
db.make_time_slot('TR1200+50', ['2 credit lecture'])
db.make_time_slot('TR1330+50', ['2 credit lecture'])
db.make_time_slot('TR1500+50', ['2 credit lecture'])
db.make_time_slot('TR1630+50', ['2 credit lecture'])
db.make_time_slot('WF0800+50', ['2 credit lecture'])
db.make_time_slot('WF0900+50', ['2 credit lecture'])
db.make_time_slot('WF1000+50', ['2 credit lecture'])
db.make_time_slot('WF1100+50', ['2 credit lecture'])
db.make_time_slot('MWF1200+50', [])
db.make_time_slot('MW0730+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
db.make_time_slot('TR0730+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
db.make_time_slot('MTRF0800+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTRF0900+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTRF1000+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTRF1100+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTRF1200+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTRF1300+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTRF1400+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTRF1500+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWF0800+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWF0900+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWF1000+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWF1100+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWF1200+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWF1300+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWF1400+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWF1500+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWR0800+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWR0900+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWR1000+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWR1100+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWR1200+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWR1300+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWR1400+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWR1500+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MWRF0800+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MWRF0900+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MWRF1000+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MWRF1100+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MWRF1200+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('MWRF1300+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MWRF1400+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MWRF1500+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('TWRF0800+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('TWRF0900+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('TWRF1000+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('TWRF1100+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('TWRF1200+50', ['4 credit bell schedule', '4 credit 4×50 bell schedule', '4 credit 4×50 extended bell schedule'])
db.make_time_slot('TWRF1300+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('TWRF1400+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('TWRF1500+50', ['4 credit 4×50 extended bell schedule'])
db.make_time_slot('MTWRF0800+50', ['5 credit bell schedule', '5 credit extended bell schedule'])
db.make_time_slot('MTWRF0900+50', ['5 credit bell schedule', '5 credit extended bell schedule'])
db.make_time_slot('MTWRF1000+50', ['5 credit bell schedule', '5 credit extended bell schedule'])
db.make_time_slot('MTWRF1100+50', ['5 credit bell schedule', '5 credit extended bell schedule'])
db.make_time_slot('MTWRF1200+50', ['5 credit bell schedule', '5 credit extended bell schedule'])
db.make_time_slot('MTWRF1300+50', ['5 credit extended bell schedule'])
db.make_time_slot('MTWRF1400+50', ['5 credit extended bell schedule'])
db.make_time_slot('MTWRF1500+50', ['5 credit extended bell schedule'])
db.make_time_slot('MTWRF1600+50', ['5 credit extended bell schedule'])
db.make_time_slot('M1030+75', [])
db.make_time_slot('R0900+75', [])
db.make_time_slot('R1330+75', [])
db.make_time_slot('T1330+75', [])
db.make_time_slot('T1500+75', [])
db.make_time_slot('W1030+75', [])
db.make_time_slot('MW1530+75', [])
db.make_time_slot('MW1645+75', [])
db.make_time_slot('MW1800+75', [])
db.make_time_slot('TR1800+75', [])
db.make_time_slot('MW1300+100', ['4 credit bell schedule', '4 credit 2×100 bell schedule'])
db.make_time_slot('MW1500+100', ['4 credit bell schedule', '4 credit 2×100 bell schedule'])
db.make_time_slot('MW1600+100', [])
db.make_time_slot('MW1630+100', [])
db.make_time_slot('MW1800+100', [])
db.make_time_slot('TR1300+100', ['4 credit bell schedule', '4 credit 2×100 bell schedule'])
db.make_time_slot('TR1500+100', ['4 credit bell schedule', '4 credit 2×100 bell schedule'])
db.make_time_slot('TR1630+100', [])
db.make_time_slot('TR1800+100', [])
db.make_time_slot('F0800+110', [])
db.make_time_slot('F0900+110', [])
db.make_time_slot('F1000+110', [])
db.make_time_slot('F1100+110', [])
db.make_time_slot('F1200+110', [])
db.make_time_slot('F1300+110', [])
db.make_time_slot('M0800+110', ['2 hour lab M0800'])
db.make_time_slot('M0900+110', ['2 hour lab M0900'])
db.make_time_slot('M1000+110', ['2 hour lab M0800'])
db.make_time_slot('M1100+110', ['2 hour lab M0900'])
db.make_time_slot('M1200+110', ['2 hour lab M0800'])
db.make_time_slot('M1300+110', ['2 hour lab M0900'])
db.make_time_slot('M1400+110', ['2 hour lab M0800'])
db.make_time_slot('M1500+110', ['2 hour lab M0900'])
db.make_time_slot('M1600+110', ['2 hour lab M0800'])
db.make_time_slot('M1700+110', [])
db.make_time_slot('R0800+110', ['2 hour lab R0800'])
db.make_time_slot('R0900+110', ['2 hour lab R0900'])
db.make_time_slot('R1000+110', ['2 hour lab R0800'])
db.make_time_slot('R1100+110', ['2 hour lab R0900'])
db.make_time_slot('R1200+110', ['2 hour lab R0800'])
db.make_time_slot('R1300+110', ['2 hour lab R0900'])
db.make_time_slot('R1400+110', ['2 hour lab R0800'])
db.make_time_slot('R1500+110', ['2 hour lab R0900'])
db.make_time_slot('R1600+110', ['2 hour lab R0800'])
db.make_time_slot('R1700+110', [])
db.make_time_slot('R1715+110', [])
db.make_time_slot('R1800+110', [])
db.make_time_slot('R1900+110', [])
db.make_time_slot('T0800+110', ['2 hour lab T0800'])
db.make_time_slot('T0900+110', ['2 hour lab T0900'])
db.make_time_slot('T1000+110', ['2 hour lab T0800'])
db.make_time_slot('T1100+110', ['2 hour lab T0900'])
db.make_time_slot('T1200+110', ['2 hour lab T0800'])
db.make_time_slot('T1300+110', ['2 hour lab T0900'])
db.make_time_slot('T1400+110', ['2 hour lab T0800'])
db.make_time_slot('T1500+110', ['2 hour lab T0900'])
db.make_time_slot('T1600+110', ['2 hour lab T0800'])
db.make_time_slot('T1700+110', [])
db.make_time_slot('T1800+110', [])
db.make_time_slot('T1900+110', [])
db.make_time_slot('W0800+110', ['2 hour lab W0800'])
db.make_time_slot('W0900+110', ['2 hour lab W0900'])
db.make_time_slot('W1000+110', ['2 hour lab W0800'])
db.make_time_slot('W1100+110', ['2 hour lab W0900'])
db.make_time_slot('W1200+110', ['2 hour lab W0800'])
db.make_time_slot('W1300+110', ['2 hour lab W0900'])
db.make_time_slot('W1400+110', ['2 hour lab W0800'])
db.make_time_slot('W1500+110', ['2 hour lab W0900'])
db.make_time_slot('W1600+110', ['2 hour lab W0800'])
db.make_time_slot('W1700+110', [])
db.make_time_slot('W1715+110', [])
db.make_time_slot('W1800+110', [])
db.make_time_slot('W1900+110', [])
db.make_time_slot('MR1100+110', [])
db.make_time_slot('MW0600+110', [])
db.make_time_slot('MW0800+110', ['4 hour lab MW0800'])
db.make_time_slot('MW0900+110', ['4 hour lab MW0900'])
db.make_time_slot('MW1000+110', ['4 hour lab MW0800'])
db.make_time_slot('MW1100+110', ['4 hour lab MW0900'])
db.make_time_slot('MW1200+110', ['4 hour lab MW0800'])
db.make_time_slot('MW1300+110', ['4 hour lab MW0900'])
db.make_time_slot('MW1400+110', ['4 hour lab MW0800'])
db.make_time_slot('MW1500+110', ['4 hour lab MW0900'])
db.make_time_slot('MW1600+110', ['4 hour lab MW0800'])
db.make_time_slot('MW1700+110', [])
db.make_time_slot('MW1800+110', [])
db.make_time_slot('TR0600+110', [])
db.make_time_slot('TR0800+110', ['4 hour lab TR0800'])
db.make_time_slot('TR0900+110', ['4 hour lab TR0900'])
db.make_time_slot('TR1000+110', ['4 hour lab TR0800'])
db.make_time_slot('TR1100+110', ['4 hour lab TR0900'])
db.make_time_slot('TR1200+110', ['4 hour lab TR0800'])
db.make_time_slot('TR1300+110', ['4 hour lab TR0900'])
db.make_time_slot('TR1400+110', ['4 hour lab TR0800'])
db.make_time_slot('TR1500+110', ['4 hour lab TR0900'])
db.make_time_slot('TR1600+110', ['4 hour lab TR0800'])
db.make_time_slot('TR1700+110', [])
db.make_time_slot('TR1800+110', [])
db.make_time_slot('F0800+115', [])
db.make_time_slot('R1200+135', [])
db.make_time_slot('R1530+150', [])
db.make_time_slot('T1630+150', [])
db.make_time_slot('W1630+150', [])
db.make_time_slot('R1330+165', [])

db.make_time_slot('M1100+170', ['3 hour lab M0800'])
db.make_time_slot('M1300+170', ['3 hour lab M1000'])
db.make_time_slot('M1930+170', [])

db.make_time_slot('T0700+170', [])
db.make_time_slot('T0800+170', ['3 hour lab T0800'])
db.make_time_slot('T0900+170', ['3 hour lab T0900'])
db.make_time_slot('T1000+170', ['3 hour lab T1000'])
db.make_time_slot('T1100+170', ['3 hour lab T0800'])
db.make_time_slot('T1200+170', ['3 hour lab T0900'])
db.make_time_slot('T1300+170', ['3 hour lab T1000'])
db.make_time_slot('T1400+170', ['3 hour lab T0800'])
db.make_time_slot('T1500+170', ['3 hour lab T0900'])
db.make_time_slot('T1600+170', [])
db.make_time_slot('T1700+170', [])
db.make_time_slot('T1800+170', [])
db.make_time_slot('T1900+170', [])
db.make_time_slot('T1930+170', [])

db.make_time_slot('W0800+170', ['3 hour lab W0800'])
db.make_time_slot('W0900+170', ['3 hour lab W0900'])
db.make_time_slot('W1000+170', ['3 hour lab W1000'])
db.make_time_slot('W1100+170', ['3 hour lab W0800'])
db.make_time_slot('W1200+170', ['3 hour lab W0900'])
db.make_time_slot('W1300+170', ['3 hour lab W1000'])
db.make_time_slot('W1330+170', [])
db.make_time_slot('W1400+170', ['3 hour lab W0800'])
db.make_time_slot('W1500+170', ['3 hour lab W0900'])
db.make_time_slot('W1600+170', [])
db.make_time_slot('W1700+170', [])
db.make_time_slot('W1930+170', [])

db.make_time_slot('R0800+170', ['3 hour lab R0800'])
db.make_time_slot('R0900+170', ['3 hour lab R0900'])
db.make_time_slot('R1000+170', ['3 hour lab R1000'])
db.make_time_slot('R1100+170', ['3 hour lab R0800'])
db.make_time_slot('R1200+170', ['3 hour lab R0900'])
db.make_time_slot('R1300+170', ['3 hour lab R1000'])
db.make_time_slot('R1400+170', ['3 hour lab R0800'])
db.make_time_slot('R1500+170', ['3 hour lab R0900'])
db.make_time_slot('R1600+170', [])
db.make_time_slot('R1630+170', [])
db.make_time_slot('R1700+170', [])
db.make_time_slot('R1900+170', [])

db.make_time_slot('F0800+170', [])
db.make_time_slot('F1100+170', [])
db.make_time_slot('F1330+170', [])
db.make_time_slot('F1400+170', [])

db.make_time_slot('MW1500+170', [])
db.make_time_slot('TR1500+170', [])
db.make_time_slot('TR1600+170', [])
db.make_time_slot('M1400+180', [])
db.make_time_slot('MWF1330+180', [])
db.make_time_slot('S1000+300', [])


#
# SET faculty
#

print('building cset faculty')

db.make_faculty('Alexander R Tye', 'Earth', 'MTWRF 0800-1700')
# F1400+170, F1400+170, R1200+170, TR1500+75

db.make_faculty("Amanda Fa'onelua", 'Math', 'TR 1300-1500')
# TR1300+100

db.make_faculty('Amber Rose Mortensen', 'Biology', 'MTWRF 0900-1700')
# MWF0900+50, MWF1000+50, MWF1100+50, TR1030+75

db.make_faculty('Andrew C Schiller', 'Engineering', 'MTWR 0900-1800, F 0900-1700')
# MW1200+75, MW1500+170, T1200+110, TR1500+170

db.make_faculty('Andrew Gregory Toth', 'Engineering', 'MW 1200-1400')
# MW1200+75

db.make_faculty('Bhuvaneswari Sambandham', 'Math', 'MTWRF 0900-1700')
# MTWF1000+50, MTWR1100+50, MW1200+75

db.make_faculty('Bing Jiang', 'Engineering', 'MTWF 0900-1700, R 0900-1800')
# F1000+110, MW1200+75, MWF0900+50, R1400+110, R1600+110

db.make_faculty('Brant A Ross', 'Engineering', 'MTWRF 0900-1700')
# MWF1330+180, MWF1330+180

db.make_faculty('Bruford P Reynolds', 'Engineering', 'MTWRF 0900-1700')
# TR1000+50, TR1400+110

db.make_faculty('Bryan K Stevens', 'Biology', 'MWF 0800-1700, TR 0700-1700')
# TR0730+75, TR0900+75, TR1030+75

db.make_faculty('Christina M Quinn', 'Chemistry', 'MWRF 0800-1700, T 0700-1700')
# R1000+170, R1300+170, T0700+170, T1000+170, T1300+170, W1300+170

db.make_faculty('Christina Pondell', 'Earth', 'MTWRF 0900-1700')
# F1000+50, M1300+170, R1330+165, T1100+110, T1300+110, TR0900+75

db.make_faculty('Christopher Kirk DeMacedo', 'Engineering', 'M 1200-2300, TWRF 1200-1700')
# M1930+170, T1200+110, T1400+110

db.make_faculty('Clare C Banks', 'Math', 'MTWRF 0800-1700')
# MTWR0800+50, MTWR1200+50

db.make_faculty('Costel Ionita', 'Math', 'MTWRF 0800-1700')
# F1100+50, MTWR0800+50, MTWR0900+50, MTWR1100+50, TR1200+75

db.make_faculty('Craig D Seegmiller', 'Math', 'MWF 0900-1700, TR 0700-1700')
# MTWR1200+50, TR0730+75, TR0900+75

db.make_faculty('Curtis B Walker', 'Biology', 'MTWRF 0800-1700')
# MW1330+75, MW1330+75, R1330+75, T1330+75, T1400+170, TR1200+75

db.make_faculty('Cutler Cowdin', 'Chemistry', 'TR 1600-1900')
# R1600+170, T1600+170

db.make_faculty('David Brent Christensen', 'Engineering', 'MTWRF 0800-1700')
# R0800+110, R1000+110, R1400+110, T1200+110

db.make_faculty('David J Burr', 'Chemistry', 'TR 1600-2200')
# R1900+170, T1600+170, T1900+170

db.make_faculty('David M Syndergaard', 'Engineering', 'MW 1200-2000, TRF 1200-1700')
# M1300+110, MW1630+75, MW1800+75

db.make_faculty('David R Black', 'Earth', 'T 1700-1900')
# T1700+110

db.make_faculty('David W Bean', 'Biology', 'MTRF 0900-1700, W 0900-1800')
# F1100+170, R1400+170, W1500+170

db.make_faculty('Dawn Lashell Kidd-Thomas', 'Math', 'TR 1200-1700')
# TR1300+100

db.make_faculty('Del William Smith', 'Biology', 'TR 1330-1900')
# TR1330+75, TR1500+50, TR1600+170

db.make_faculty('Diana L Reese', 'Chemistry', 'MTWRF 0900-1700')
# MTWR0900+50, MTWR1000+50, MTWRF1200+50, MTWRF1600+50

db.make_faculty('Divya Singh', 'Engineering', 'MW 0900-1800, TRF 0900-1700')
# MW1000+110, MW1500+75, MW1630+75, T1200+110

db.make_faculty('Donald H Warner', 'Biology', 'MW 1500-1700')
# MW1500+75

db.make_faculty('Douglas J Sainsbury', 'Biology', 'MTWRF 0800-1700')
# MTWRF0800+50, TR1200+75, W1200+50

db.make_faculty('Elizabeth Karen Ludlow', 'Math', 'MW 1300-1700')
# MW1300+100, MW1500+75

db.make_faculty("Erin E O'Brien", 'Biology', 'MRF 1200-1700, TW 1200-1800')
# MW1200+75, T1500+170, W1500+170

db.make_faculty('Gabriela Chilom', 'Chemistry', 'MTWF 0800-1700, R 0800-1800')
# MTWR0800+50, MTWR1400+50, MTWRF1500+50, MWF1000+50, R1500+170

db.make_faculty('Geoffrey Smith', 'Biology', 'MTWRF 0900-1700')
# MTWR1100+50, TR1500+75

db.make_faculty('Glorimar L Aponte-Kline', 'Biology', 'MTWRF 0900-1700')
# TR0900+75, TR1030+75, TR1330+75

db.make_faculty('Greg L Melton', 'Earth', 'MTWRF 0900-1700')
# MW1330+75, MW1500+75, T1200+170, TR0900+75, W0900+110

db.make_faculty('Hugo Elio Angeles', 'Earth', 'TR 1800-2000')
# TR1800+75

db.make_faculty('Hung Yu Shih', 'Biology', 'TWR 1200-1700')
# T1300+110, T1300+110, T1500+50, T1600+50, W1330+170

db.make_faculty('Jacson Parker', 'Chemistry', 'TR 1600-1900')
# R1600+170, T1600+170

db.make_faculty('James David Meidell', 'Biology', 'MW 1630-1800, TR 1630-2000')
# MW1630+75, R1700+170

db.make_faculty('James P Fitzgerald', 'Math', 'MTWRF 0800-1200')
# MWF0800+50, MWF0900+50, MWF1000+50

db.make_faculty('Jameson C Hardy', 'Math', 'MTWRF 0900-1700')
# MTWR0900+50, MTWRF1000+50, MW1200+75, TR1200+75

db.make_faculty('Janice M Hayden', 'Earth', 'TWR 0900-1700')
# TR0900+75, W1100+170

db.make_faculty('Jared M Hancock', 'Chemistry', 'MTWRF 0800-1700')
# M1100+110, MTWR0800+50, MTWR0900+50, MTWR1400+50, W1000+170

db.make_faculty('Jeffrey Anderson', 'Engineering', 'MW 0900-1800, TRF 0900-1700')
# MW1630+75, T1400+110, TR0900+75

db.make_faculty('Jeffrey P Harrah', 'Math', 'MRF 0800-1700, TW 0800-1900')
# T1630+150, TR1030+75, TR1200+75, TR1330+75, W1630+150

db.make_faculty('Jeffrey V Yule', 'Biology', 'MTWRF 0900-1700')
# M1030+75, MWF1100+50, TR1030+75, TR1030+75, W1030+75

db.make_faculty('Jennifer A Meyer', 'Chemistry', 'MTWRF 0900-1700')
# MW1200+75, MW1330+75, R1300+170, T1300+170

db.make_faculty('Jennifer L Ciaccio', 'Biology', 'MTWRF 0900-1700')
# MTRF1200+50, MWF0900+50, R0900+75, W1200+170

db.make_faculty('Jerald D Harris', 'Earth', 'MTWF 0900-1700, R 0900-2000')
# MWF1000+50, MWF1100+50, MWF1100+50, R1000+50, R1630+170, TR1030+75

db.make_faculty('Jeremy W Bakelar', 'Biology', 'MWRF 0900-1700, T 0900-1800')
# MW1500+75, MWF1100+50, T0900+170, T1500+170, TR1300+110

db.make_faculty('Jesse William Breinholt', 'Biology', 'TR 1500-1700')
# TR1500+75

db.make_faculty('Jie Liu', 'Math', 'MTWRF 0900-1700')
# T1500+75, TR1030+75, TR1200+75, TR1330+75

db.make_faculty('John E Wolfe', 'Biology', 'MWF 1100-1200')
# MWF1100+50

db.make_faculty('Jose C Saraiva', 'Engineering', 'MF 1600-1700, T 1600-1800, W 1600-2300, R 1600-2000')
# R1600+110, R1800+110, T1600+110, W1930+170

db.make_faculty('Joseph B Platt', 'Biology', 'R 1100-1400')
# R1100+170

db.make_faculty('Kameron J Eves', 'Engineering', 'MTWF 0900-1700, R 0900-1800')
# MW1500+75, MWF1100+50, R1600+110, TR1030+75

db.make_faculty('Karen L Bauer', 'Biology', 'MTWRF 0800-1700')
# MTWF1000+50, MTWF1100+50, MWF0800+50, TR1500+75

db.make_faculty('Kathryn E Ott', 'Math', 'MW 1300-1700')
# MW1300+100

db.make_faculty('Kerby Robinson', 'Biology', 'F 1330-1700')
# F1330+170

db.make_faculty('Kim C Jolley', 'Biology', 'MW 1300-1900')
# MW1300+110, MW1700+110

db.make_faculty('Marius Van der Merwe', 'Earth', 'MTRF 0900-1700, W 0900-1900')
# MWF1000+50, T1200+170, W0900+50, W1800+50

db.make_faculty('Mark L Dickson', 'Biology', 'MTWRF 1530-1800')
# R1530+150

db.make_faculty('Marshall Topham', 'Earth', 'MW 1330-1700')
# MW1330+75

db.make_faculty('Martina Gaspari', 'Biology', 'MTWRF 0800-1700')
# MR1100+110, MW1330+75, MWF0900+50, MWF1000+50, R0800+170

db.make_faculty('Marzieh Ghasemi', 'Earth', 'MTWRF 0900-1700')
# MW1200+75, MWF1000+50, TR1200+75, TR1500+75

db.make_faculty('Md Sazib Hasan', 'Math', 'TR 0900-1200')
# TR0900+75, TR1030+75

db.make_faculty('Megan R Liljenquist', 'Chemistry', 'TWR 1500-1900')
# R1600+170, W1500+170

db.make_faculty('Megen E Kepas', 'Biology', 'MTWRF 0900-1700')
# MW1330+75, MW1500+75, R1200+135

db.make_faculty('Michael N Paxman', 'Math', 'TR 1630-1900')
# TR1630+100

db.make_faculty('Nathan St Andre', 'Biology', 'TR 1200-1700')
# TR1200+75

db.make_faculty('Nikell Dodge', 'Biology', 'TR 1630-1800')
# TR1630+75

db.make_faculty('Odean Bowler', 'Math', 'MTWRF 1500-1700')
# MW1500+100, TR1500+100

db.make_faculty('Paul H Shirley', 'Chemistry', 'T 1600-2200')
# T1600+170, T1900+170

db.make_faculty('Paula Manuele Temple', 'Math', 'MTWRF 0900-1700')
# MTWR1200+50, MW1300+100, MW1500+75, TR1300+100

db.make_faculty('Randy Klabacka', 'Biology', 'MTWRF 0900-1700')
# MW1330+50, MWF0900+50, MWF0900+50, R0900+50, T0900+50, TR1330+75

db.make_faculty('Rick L Peirce', 'Engineering', 'T 1930-2300')
# T1930+170

db.make_faculty('Rico Del Sesto', 'Chemistry', 'MTWRF 0900-1200')
# MTWRF0900+50, MTWRF1000+50, MTWRF1100+50

db.make_faculty('Rita Rae Osborn', 'Biology', 'M 0800-0900')
# M0800+50

db.make_faculty('Robert T Reimer', 'Math', 'MW 1630-1800')
# MW1630+75

db.make_faculty('Ross C Decker', 'Math', 'TR 0900-1200')
# TR0900+75, TR1030+75

db.make_faculty('Russell C Reid', 'Engineering', 'MTWF 0800-1700, R 0800-1800')
# MTWF0900+50, MTWF0900+50, MW1500+75, R0800+110, R1000+110, R1200+110, R1400+110, R1600+110

db.make_faculty('Ryan C McConnell', 'Math', 'TR 1630-1800')
# TR1630+75

db.make_faculty('Sai C Radavaram', 'Engineering', 'MWF 0800-1700, TR 0800-1800')
# F0800+115, MW1330+75, MWF1100+50, T0800+110, TR1630+75

db.make_faculty('Samuel K Tobler', 'Engineering', 'MTWRF 0900-1700')
# MTWF1300+50, MTWF1400+50

db.make_faculty('Sarah Morgan Black', 'Chemistry', 'TR 0900-1700')
# TR1030+75, TR1330+75

db.make_faculty('Scott A Skeen', 'Engineering', 'MTWRF 0800-1700')
# M0800+50, MW1200+75, MW1330+75, MWF1000+50, R0800+110, R1200+110, TR1500+75

db.make_faculty('Scott B Griffin', 'Biology', 'MTWRF 0900-1700')
# F1330+170, MW1200+75

db.make_faculty('Scott E Bulloch', 'Biology', 'R 1600-1900')
# R1600+170

db.make_faculty('Scott Patrick Hicks', 'Math', 'MW 1600-2000')
# MW1600+100, MW1800+100

db.make_faculty('Steven K Sullivan', 'Engineering', 'MTWRF 0800-1200')
# MWRF0800+50, MWRF1000+50, MWRF1100+50

db.make_faculty('Steven McKay Sullivan', 'Math', 'MTWRF 0900-1700')
# MTWR0900+50, MWF1000+50, TR1030+75

db.make_faculty('Teisha Richan', 'Chemistry', 'MTWRF 0900-1700')
# R1000+170, R1300+170, T0900+170, T1200+170, W0900+170, W1200+170

db.make_faculty('Trevor K Johnson', 'Math', 'MTWRF 0900-1700')
# MTWR1200+50, MW1330+75

db.make_faculty('Tye K Rogers', 'Math', 'MTWRF 0800-1700')
# MTWR0800+50, MTWR1000+50, MWF1100+50, TR1330+75

db.make_faculty('Vinodh Kumar Chellamuthu', 'Math', 'MW 1500-1800')
# MW1500+100, MW1645+75

db.make_faculty('Violeta Adina Ionita', 'Math', 'MTWRF 0800-1700')
# MTWR0800+50, MTWR0900+50, MTWR1100+50, MTWR1200+50

db.make_faculty('Wendy E Schatzberg', 'Chemistry', 'MWRF 0900-1700, T 0900-1900')
# F1200+50, MTWR1000+50, MTWR1100+50, MTWRF1200+50, T1600+170

db.make_faculty('Zhenyu Jin', 'Earth', 'MTWRF 0900-1700')
# MW1200+75, MW1330+75, T1200+170, TR1030+75, W0900+110

#
# SET sections
#

print('building cset sections')

# BIOL 1010-01: General Biology (LS)
# assigned to BROWN 201 at TR0730+75
db.make_section('BIOL 1010-01', ['Brown 201', 'Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Bryan K Stevens', ['BIOL 1010-01'])

# BIOL 1010-02: General Biology (LS)
# assigned to BROWN 201 at TR0900+75
db.make_section('BIOL 1010-02', ['Brown 201', 'Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Bryan K Stevens', ['BIOL 1010-02'])

# BIOL 1010-03: General Biology (LS)
# assigned to SET 301 at MWF0800+50
db.make_section('BIOL 1010-03', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Karen L Bauer', ['BIOL 1010-03'])

# BIOL 1010-04: General Biology (LS)
# assigned to COE 121 at MWF1000+50
db.make_section('BIOL 1010-04', ['COE 121', 'Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Martina Gaspari', ['BIOL 1010-04'])

# BIOL 1010-05: General Biology: Supplemental Instruction (LS)
# assigned to SET 106 at TR1030+75
db.make_section('BIOL 1010-05', ['Science large lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jeffrey V Yule', ['BIOL 1010-05'])

# BIOL 1010-05-alt: General Biology: Supplemental Instruction (LS)
# assigned to SNOW 113 at W1030+75
db.make_section('BIOL 1010-05-SI', ['Snow 113', 'Science small lecture', 'W1030+75'])

# BIOL 1010-06: General Biology (LS)
# assigned to BROWN 201 at MWF1100+50
db.make_section('BIOL 1010-06', ['Brown 201', 'Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jeffrey V Yule', ['BIOL 1010-06'])

# BIOL 1010-07: General Biology (LS)
# assigned to SET 105 at TR1200+75
db.make_section('BIOL 1010-07', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Nathan St Andre', ['BIOL 1010-07'])

# BIOL 1010-08: General Biology (LS)
# assigned to SNOW 151 at TR1330+75
db.make_section('BIOL 1010-08', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Del William Smith', ['BIOL 1010-08'])

# BIOL 1010-09: General Biology (LS)
# assigned to SET 420 at MW1630+75
db.make_section('BIOL 1010-09', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('James David Meidell', ['BIOL 1010-09'])

# BIOL 1010-10: General Biology (LS)
# assigned to SET 301 at TR1630+75
db.make_section('BIOL 1010-10', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Nikell Dodge', ['BIOL 1010-10'])

# BIOL 1010-11: General Biology (LS)
# assigned to SNOW 113 at M1030+75
db.make_section('BIOL 1010-11-SI', ['Snow 113', 'Science small lecture', 'M1030+75'])

# BIOL 1010-11-alt: General Biology (LS)
# assigned to SET 106 at TR1030+75
db.make_section('BIOL 1010-11', []) # 'Science large lecture', '3 credit bell schedule'
# ./edit assign-faculty-sections 'Jeffrey V Yule', 'BIOL 1010-11'
# cross-listed with BIOL 1010-05

# BIOL 1015-04: General Biology Lab (LAB)
# assigned to SET 312 at T1100+170
db.make_section('BIOL 1015-04', ['SET 312', '3 hour lab T0800'])

# BIOL 1015-05: General Biology Lab (LAB)
# assigned to SET 312 at W1100+170
db.make_section('BIOL 1015-05', ['SET 312', '3 hour lab W0800'])

# BIOL 1015-07: General Biology Lab (LAB)
# assigned to SET 312 at T1400+170
db.make_section('BIOL 1015-07', ['SET 312', '3 hour lab T0800'])

# BIOL 1015-51: General Biology Lab (LAB)
# assigned to SET 312 at T1700+170
db.make_section('BIOL 1015-51', ['SET 312', 'T1700+170'])

# BIOL 1200-01: Human Biology (LS)
# assigned to BROWN 201 at TR1030+75
db.make_section('BIOL 1200-01', ['Brown 201', 'Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Amber Rose Mortensen', ['BIOL 1200-01'])

# BIOL 1200-02: Human Biology (LS)
# assigned to SET 105 at TR1500+75
db.make_section('BIOL 1200-02', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Karen L Bauer', ['BIOL 1200-02'])

# BIOL 1610-01: Principles of Biology I (LS)
# assigned to SET 106 at MTWRF0800+50
db.make_section('BIOL 1610-01', ['Science large lecture', '5 credit bell schedule'])
db.assign_faculty_sections('Douglas J Sainsbury', ['BIOL 1610-01'])

# BIOL 1610-02: Principles of Biology I (LS)
# assigned to SET 105 at MTWF1100+50
db.make_section('BIOL 1610-02', ['Science medium lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Karen L Bauer', ['BIOL 1610-02'])

# BIOL 1615-01: Principles of Biology I Lab (LAB)
# assigned to SET 309 at T0800+170
db.make_section('BIOL 1615-01', ['SET 309', '3 hour lab T0800'])

# BIOL 1615-02: Principles of Biology I Lab (LAB)
# assigned to SET 309 at W0800+170
db.make_section('BIOL 1615-02', ['SET 309', '3 hour lab W0800'])

# BIOL 1615-04: Principles of Biology I Lab (LAB)
# assigned to SET 309 at F0800+170
db.make_section('BIOL 1615-04', ['SET 309', 'F0800+170'])

# BIOL 1615-05: Principles of Biology I Lab (LAB)
# assigned to SET 309 at T1100+170
db.make_section('BIOL 1615-05', ['SET 309', '3 hour lab T0800'])

# BIOL 1615-06: Principles of Biology I Lab (LAB)
# assigned to SET 309 at W1100+170
db.make_section('BIOL 1615-06', ['SET 309', '3 hour lab W0800'])

# BIOL 1615-07: Principles of Biology I Lab (LAB)
# assigned to SET 309 at R1100+170
db.make_section('BIOL 1615-07', ['SET 309', '3 hour lab R0800'])

# BIOL 1615-08: Principles of Biology I Lab (LAB)
# assigned to SET 309 at F1100+170
db.make_section('BIOL 1615-08', ['SET 309', 'F1100+170'])

# BIOL 1615-09: Principles of Biology I Lab (LAB)
# assigned to SET 309 at T1400+170
db.make_section('BIOL 1615-09', ['SET 309', '3 hour lab T0800'])

# BIOL 1615-10: Principles of Biology I Lab (LAB)
# assigned to SET 309 at W1400+170
db.make_section('BIOL 1615-10', ['SET 309', '3 hour lab W0800'])

# BIOL 1615-11: Principles of Biology I Lab (LAB)
# assigned to SET 309 at R1400+170
db.make_section('BIOL 1615-11', ['SET 309', '3 hour lab R0800'])

# BIOL 1615-12: Principles of Biology I Lab (LAB)
# assigned to SET 309 at F1400+170
db.make_section('BIOL 1615-12', ['SET 309', 'F1400+170'])

# BIOL 1615-50: Principles of Biology I Lab (LAB)
# assigned to SET 309 at T1700+170
db.make_section('BIOL 1615-50', ['SET 309', 'T1700+170'])

# BIOL 1615-51: Principles of Biology I Lab (LAB)
# assigned to SET 309 at W1700+170
db.make_section('BIOL 1615-51', ['SET 309', 'W1700+170'])

# BIOL 1615-52: Principles of Biology I Lab (LAB)
# assigned to SET 309 at R1700+170
db.make_section('BIOL 1615-52', ['SET 309', 'R1700+170'])

# BIOL 1620-01: Principles of Biology II
# assigned to SET 105 at MTWF1000+50
db.make_section('BIOL 1620-01', ['Science medium lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Karen L Bauer', ['BIOL 1620-01'])

# BIOL 1620-02: Principles of Biology II
# assigned to SET 106 at MTRF1200+50
db.make_section('BIOL 1620-02', ['Science large lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Jennifer L Ciaccio', ['BIOL 1620-02'])

# BIOL 1620-03: Principles of Biology II (HONORS)
# assigned to SET 216 at MTWR1100+50
db.make_section('BIOL 1620-03', ['SET 216', '4 credit bell schedule'])
db.assign_faculty_sections('Geoffrey Smith', ['BIOL 1620-03'])

# BIOL 1625-01: Principles of Biology II Lab
# assigned to SET 318 at R0800+170
db.make_section('BIOL 1625-01', ['SET 318', '3 hour lab R0800'])

# BIOL 1625-02: Principles of Biology II Lab
# assigned to SET 318 at R1100+170
db.make_section('BIOL 1625-02', ['SET 318', '3 hour lab R0800'])
db.assign_faculty_sections('Joseph B Platt', ['BIOL 1625-02'])

# BIOL 1625-03: Principles of Biology II Lab
# assigned to SET 318 at W1200+170
db.make_section('BIOL 1625-03', ['SET 318', '3 hour lab W0900'])
db.assign_faculty_sections('Jennifer L Ciaccio', ['BIOL 1625-03'])

# BIOL 1625-04: Principles of Biology II Lab
# assigned to SET 318 at R1400+170
db.make_section('BIOL 1625-04', ['SET 318', '3 hour lab R0800'])
db.assign_faculty_sections('David W Bean', ['BIOL 1625-04'])

# BIOL 1625-05: Principles of Biology II Lab
# assigned to SET 318 at F1100+170
db.make_section('BIOL 1625-05', ['SET 318', 'F1100+170'])
db.assign_faculty_sections('David W Bean', ['BIOL 1625-05'])

# BIOL 1625-06: Principles of Biology II Lab
# assigned to SET 318 at W1500+170
db.make_section('BIOL 1625-06', ['SET 318', '3 hour lab W0900'])
db.assign_faculty_sections('David W Bean', ['BIOL 1625-06'])

# BIOL 1625-50: Principles of Biology II Lab
# assigned to SET 318 at R1700+170
db.make_section('BIOL 1625-50', ['SET 318', 'R1700+170'])
db.assign_faculty_sections('James David Meidell', ['BIOL 1625-50'])

# BIOL 2060-01: Principles of Microbiology
# assigned to SET 105 at MW1500+75
db.make_section('BIOL 2060-01', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jeremy W Bakelar', ['BIOL 2060-01'])

# BIOL 2065-01: Principles of Microbiology Lab
# assigned to SET 304 at MW1300+110
db.make_section('BIOL 2065-01', ['SET 304', '4 hour lab MW0900'])
db.assign_faculty_sections('Kim C Jolley', ['BIOL 2065-01'])

# BIOL 2065-02: Principles of Microbiology Lab
# assigned to SET 304 at MW1700+110
db.make_section('BIOL 2065-02', ['SET 304', 'MW1700+110'])
db.assign_faculty_sections('Kim C Jolley', ['BIOL 2065-02'])

# BIOL 2300-01: Fundamentals of Bioinformatics
# assigned to SET 216 at MW1330+50
db.make_section('BIOL 2300-01', ['SET 216', '2 credit lecture'])
db.assign_faculty_sections('Randy Klabacka', ['BIOL 2300-01'])

# BIOL 2320-01: Human Anatomy
# assigned to BROWN 201 at MWF1000+50
db.make_section('BIOL 2320-01', ['Brown 201', 'Science small lecture', '3 credit bell schedule'])

# BIOL 2320-02: Human Anatomy
# assigned to SET 301 at MW1200+75
db.make_section('BIOL 2320-02', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Scott B Griffin', ['BIOL 2320-02'])

# BIOL 2320-04: Human Anatomy: Supplemental Instruction
# assigned to SET 301 at MW1330+75
db.make_section('BIOL 2320-04', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Curtis B Walker', ['BIOL 2320-04'])

# BIOL 2320-04-alt: Human Anatomy: Supplemental Instruction
# assigned to SET 105 at T1330+75
db.make_section('BIOL 2320-04-SI', ['Science medium lecture', 'T1330+75'])

# BIOL 2320-05: Human Anatomy
# assigned to SET 301 at TR1030+75
db.make_section('BIOL 2320-05', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Glorimar L Aponte-Kline', ['BIOL 2320-05'])

# BIOL 2320-07: Human Anatomy
# assigned to SET 301 at TR1330+75
db.make_section('BIOL 2320-07', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Glorimar L Aponte-Kline', ['BIOL 2320-07'])

# BIOL 2320-08: Human Anatomy: Supplemental Instruction
# assigned to SET 301 at MW1330+75
db.make_section('BIOL 2320-08', []) # 'Science medium lecture', '3 credit bell schedule'
# ./edit assign-faculty-sections 'Curtis B Walker', 'BIOL 2320-08'
# cross-listed with BIOL 2320-04

# BIOL 2320-08-alt: Human Anatomy: Supplemental Instruction
# assigned to SET 105 at R1330+75
db.make_section('BIOL 2320-08-SI', ['Science medium lecture', 'R1330+75'])

# BIOL 2325-01: Human Anatomy Lab
# assigned to SET 213 at MW0600+110
db.make_section('BIOL 2325-01', ['SET 213', 'MW0600+110'])

# BIOL 2325-02: Human Anatomy Lab
# assigned to SET 215 at TR0600+110
db.make_section('BIOL 2325-02', ['SET 215', 'TR0600+110'])

# BIOL 2325-03: Human Anatomy Lab
# assigned to SET 213 at MW0800+110
db.make_section('BIOL 2325-03', ['SET 213', '4 hour lab MW0800'])

# BIOL 2325-04: Human Anatomy Lab
# assigned to SET 215 at MW0800+110
db.make_section('BIOL 2325-04', ['SET 215', '4 hour lab MW0800'])

# BIOL 2325-05: Human Anatomy Lab
# assigned to SET 213 at TR0800+110
db.make_section('BIOL 2325-05', ['SET 213', '4 hour lab TR0800'])

# BIOL 2325-06: Human Anatomy Lab
# assigned to SET 215 at TR0800+110
db.make_section('BIOL 2325-06', ['SET 215', '4 hour lab TR0800'])

# BIOL 2325-07: Human Anatomy Lab
# assigned to SET 213 at MW1000+110
db.make_section('BIOL 2325-07', ['SET 213', '4 hour lab MW0800'])

# BIOL 2325-08: Human Anatomy Lab
# assigned to SET 215 at MW1000+110
db.make_section('BIOL 2325-08', ['SET 215', '4 hour lab MW0800'])

# BIOL 2325-09: Human Anatomy Lab
# assigned to SET 213 at TR1000+110
db.make_section('BIOL 2325-09', ['SET 213', '4 hour lab TR0800'])

# BIOL 2325-10: Human Anatomy Lab
# assigned to SET 215 at TR1000+110
db.make_section('BIOL 2325-10', ['SET 215', '4 hour lab TR0800'])

# BIOL 2325-11: Human Anatomy Lab
# assigned to SET 213 at MW1200+110
db.make_section('BIOL 2325-11', ['SET 213', '4 hour lab MW0800'])

# BIOL 2325-12: Human Anatomy Lab
# assigned to SET 215 at MW1200+110
db.make_section('BIOL 2325-12', ['SET 215', '4 hour lab MW0800'])

# BIOL 2325-13: Human Anatomy Lab
# assigned to SET 213 at TR1200+110
db.make_section('BIOL 2325-13', ['SET 213', '4 hour lab TR0800'])

# BIOL 2325-14: Human Anatomy Lab
# assigned to SET 215 at TR1200+110
db.make_section('BIOL 2325-14', ['SET 215', '4 hour lab TR0800'])

# BIOL 2325-15: Human Anatomy Lab
# assigned to SET 213 at MW1400+110
db.make_section('BIOL 2325-15', ['SET 213', '4 hour lab MW0800'])

# BIOL 2325-16: Human Anatomy Lab
# assigned to SET 215 at MW1400+110
db.make_section('BIOL 2325-16', ['SET 215', '4 hour lab MW0800'])

# BIOL 2325-17: Human Anatomy Lab
# assigned to SET 213 at TR1400+110
db.make_section('BIOL 2325-17', ['SET 213', '4 hour lab TR0800'])

# BIOL 2325-18: Human Anatomy Lab
# assigned to SET 215 at TR1400+110
db.make_section('BIOL 2325-18', ['SET 215', '4 hour lab TR0800'])

# BIOL 2325-19: Human Anatomy Lab
# assigned to SET 213 at MW1600+110
db.make_section('BIOL 2325-19', ['SET 213', '4 hour lab MW0800'])

# BIOL 2325-20: Human Anatomy Lab
# assigned to SET 215 at MW1600+110
db.make_section('BIOL 2325-20', ['SET 215', '4 hour lab MW0800'])

# BIOL 2325-21: Human Anatomy Lab
# assigned to SET 213 at TR1600+110
db.make_section('BIOL 2325-21', ['SET 213', '4 hour lab TR0800'])

# BIOL 2325-22: Human Anatomy Lab
# assigned to SET 215 at TR1600+110
db.make_section('BIOL 2325-22', ['SET 215', '4 hour lab TR0800'])

# BIOL 2325-50: Human Anatomy Lab
# assigned to SET 213 at MW1800+110
db.make_section('BIOL 2325-50', ['SET 213', 'MW1800+110'])

# BIOL 2325-51: Human Anatomy Lab
# assigned to SET 215 at MW1800+110
db.make_section('BIOL 2325-51', ['SET 215', 'MW1800+110'])

# BIOL 2325-52: Human Anatomy Lab
# assigned to SET 213 at TR1800+110
db.make_section('BIOL 2325-52', ['SET 213', 'TR1800+110'])

# BIOL 2325-53: Human Anatomy Lab
# assigned to SET 215 at TR1800+110
db.make_section('BIOL 2325-53', ['SET 215', 'TR1800+110'])

# BIOL 2420-01: Human Physiology
# assigned to SET 106 at MWF0900+50
db.make_section('BIOL 2420-01', ['Science large lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Amber Rose Mortensen', ['BIOL 2420-01'])

# BIOL 2420-02: Human Physiology
# assigned to SET 106 at MWF1000+50
db.make_section('BIOL 2420-02', ['Science large lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Amber Rose Mortensen', ['BIOL 2420-02'])

# BIOL 2420-03: Human Physiology
# assigned to SET 106 at MWF1100+50
db.make_section('BIOL 2420-03', ['Science large lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Amber Rose Mortensen', ['BIOL 2420-03'])

# BIOL 2420-04: Human Physiology
# assigned to SET 301 at MW1500+75
db.make_section('BIOL 2420-04', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Megen E Kepas', ['BIOL 2420-04'])

# BIOL 2420-05: Human Physiology
# assigned to SET 301 at TR1500+75
db.make_section('BIOL 2420-05', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Geoffrey Smith', ['BIOL 2420-05'])

# BIOL 2425-01: Human Physiology Lab
# assigned to SET 214 at T0900+110
db.make_section('BIOL 2425-01', ['SET 214', '2 hour lab T0900'])

# BIOL 2425-02: Human Physiology Lab
# assigned to SET 214 at W0900+110
db.make_section('BIOL 2425-02', ['SET 214', '2 hour lab W0900'])

# BIOL 2425-03: Human Physiology Lab
# assigned to SET 214 at R0900+110
db.make_section('BIOL 2425-03', ['SET 214', '2 hour lab R0900'])

# BIOL 2425-04: Human Physiology Lab
# assigned to SET 214 at F0900+110
db.make_section('BIOL 2425-04', ['SET 214', 'F0900+110'])

# BIOL 2425-05: Human Physiology Lab
# assigned to SET 214 at T1100+110
db.make_section('BIOL 2425-05', ['SET 214', '2 hour lab T0900'])

# BIOL 2425-06: Human Physiology Lab
# assigned to SET 214 at W1100+110
db.make_section('BIOL 2425-06', ['SET 214', '2 hour lab W0900'])

# BIOL 2425-07: Human Physiology Lab
# assigned to SET 214 at R1100+110
db.make_section('BIOL 2425-07', ['SET 214', '2 hour lab R0900'])

# BIOL 2425-08: Human Physiology Lab
# assigned to SET 214 at F1100+110
db.make_section('BIOL 2425-08', ['SET 214', 'F1100+110'])

# BIOL 2425-09: Human Physiology Lab
# assigned to SET 214 at T1300+110
db.make_section('BIOL 2425-09', ['SET 214', '2 hour lab T0900'])

# BIOL 2425-10: Human Physiology Lab
# assigned to SET 214 at W1300+110
db.make_section('BIOL 2425-10', ['SET 214', '2 hour lab W0900'])

# BIOL 2425-11: Human Physiology Lab
# assigned to SET 214 at R1300+110
db.make_section('BIOL 2425-11', ['SET 214', '2 hour lab R0900'])

# BIOL 2425-12: Human Physiology Lab
# assigned to SET 214 at F1300+110
db.make_section('BIOL 2425-12', ['SET 214', 'F1300+110'])

# BIOL 2425-13: Human Physiology Lab
# assigned to SET 214 at T1500+110
db.make_section('BIOL 2425-13', ['SET 214', '2 hour lab T0900'])

# BIOL 2425-14: Human Physiology Lab
# assigned to SET 214 at W1500+110
db.make_section('BIOL 2425-14', ['SET 214', '2 hour lab W0900'])

# BIOL 2425-15: Human Physiology Lab
# assigned to SET 214 at R1500+110
db.make_section('BIOL 2425-15', ['SET 214', '2 hour lab R0900'])

# BIOL 2425-50: Human Physiology Lab
# assigned to SET 214 at T1700+110
db.make_section('BIOL 2425-50', ['SET 214', 'T1700+110'])

# BIOL 2425-51: Human Physiology Lab
# assigned to SET 214 at W1700+110
db.make_section('BIOL 2425-51', ['SET 214', 'W1700+110'])

# BIOL 2991R-01A: Careers in Biology
# assigned to SET 501 at W1200+50
db.make_section('BIOL 2991R-01A', ['SET 501', '1 credit extended bell schedule'])
db.assign_faculty_sections('Douglas J Sainsbury', ['BIOL 2991R-01A'])

# BIOL 3000R-09A: Advanced Utah Health Scholars Students
# xlist entry: HO04
# assigned to SET 105 at M0800+50
db.make_section('BIOL 3000R-09A', ['Science medium lecture', '1 credit bell schedule'])
db.assign_faculty_sections('Rita Rae Osborn', ['BIOL 3000R-09A'])

# BIOL 3010-01: Evolution
# assigned to SET 301 at MWF1100+50
db.make_section('BIOL 3010-01', ['Science medium lecture', '3 credit bell schedule'])

# BIOL 3010-01-alt: Evolution
# assigned to SET 301 at T1200+50
db.make_section('BIOL 3010-01-SI', ['Science medium lecture', '1 credit extended bell schedule'])

# BIOL 3010-02: Evolution
# assigned to SET 301 at MWF1100+50
db.make_section('BIOL 3010-02', []) # 'Science medium lecture', '3 credit bell schedule'
# cross-listed with BIOL 3010-01

# BIOL 3010-02-alt: Evolution
# assigned to SET 301 at R1200+50
db.make_section('BIOL 3010-02-SI', ['Science medium lecture', '1 credit extended bell schedule'])

# BIOL 3030-01: Principles of Genetics: Supplemental Instruction
# assigned to SET 301 at MWF0900+50
db.make_section('BIOL 3030-01', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Randy Klabacka', ['BIOL 3030-01'])

# BIOL 3030-01-alt: Principles of Genetics: Supplemental Instruction
# assigned to SET 301 at T0900+50
db.make_section('BIOL 3030-01-SI', ['Science medium lecture', '1 credit bell schedule'])

# BIOL 3030-02: Genetics
# assigned to SET 301 at MWF0900+50
db.make_section('BIOL 3030-02', []) # 'Science medium lecture', '3 credit bell schedule'
# ./edit assign-faculty-sections 'Randy Klabacka', 'BIOL 3030-02'
# cross-listed with BIOL 3030-01

# BIOL 3030-02-alt: Genetics
# assigned to SET 301 at R0900+50
db.make_section('BIOL 3030-02-SI', ['Science medium lecture', '1 credit bell schedule'])

# BIOL 3040-01: General Ecology
# assigned to SET 301 at MWF1000+50
db.make_section('BIOL 3040-01', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Marius Van der Merwe', ['BIOL 3040-01'])

# BIOL 3045-01: General Ecology Lab
# assigned to SET 216 at T1200+170
db.make_section('BIOL 3045-01', ['SET 216', '3 hour lab T0900'])
db.assign_faculty_sections('Marius Van der Merwe', ['BIOL 3045-01'])

# BIOL 3100-01: Bioethics
# xlist entry: SC0B
# assigned to HCC 476 at MWF1100+50
db.make_section('BIOL 3100-01', ['HCC 476', '3 credit bell schedule'])
db.assign_faculty_sections('John E Wolfe', ['BIOL 3100-01'])

# BIOL 3110-01: Scientific Writing
# assigned to SET 408 at R0900+75
db.make_section('BIOL 3110-01', ['SET 408', 'R0900+75'])
db.assign_faculty_sections('Jennifer L Ciaccio', ['BIOL 3110-01'])

# BIOL 3150-01: Biostatistics & the Sci Method
# assigned to SET 106 at MW1330+75
db.make_section('BIOL 3150-01', ['Science large lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Megen E Kepas', ['BIOL 3150-01'])

# BIOL 3155-01: Scientific Method and Experimental Design
# assigned to SET 216 at R1200+135
db.make_section('BIOL 3155-01', ['SET 216', 'R1200+135'])
db.assign_faculty_sections('Megen E Kepas', ['BIOL 3155-01'])

# BIOL 3155-02: Scientific Method and Experimental Design
# assigned to SET 216 at T1500+170
db.make_section('BIOL 3155-02', ['SET 216', '3 hour lab T0900'])
db.assign_faculty_sections("Erin E O'Brien", ['BIOL 3155-02'])

# BIOL 3230R-01: Cadaver Practicum
# assigned to SET 213 at F1330+170
db.make_section('BIOL 3230R-01', ['SET 213', 'F1330+170'])
db.assign_faculty_sections('Scott B Griffin', ['BIOL 3230R-01'])

# BIOL 3230R-02: Cadaver Practicum
# assigned to SET 215 at F1330+170
db.make_section('BIOL 3230R-02', ['SET 215', 'F1330+170'])
db.assign_faculty_sections('Kerby Robinson', ['BIOL 3230R-02'])

# BIOL 3250-01: Cancer Biology
# assigned to SET 319 at MW1330+75
db.make_section('BIOL 3250-01', ['SET 319', '3 credit bell schedule'])
db.assign_faculty_sections('Martina Gaspari', ['BIOL 3250-01'])

# BIOL 3300-01: Introduction to Bioinformatics
# assigned to SET 501 at TR1500+75
db.make_section('BIOL 3300-01', ['SET 501', '3 credit bell schedule'])
db.assign_faculty_sections('Jesse William Breinholt', ['BIOL 3300-01'])

# BIOL 3420-01: Advanced Human Physiology
# assigned to SNOW 128 at TR0900+75
db.make_section('BIOL 3420-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Glorimar L Aponte-Kline', ['BIOL 3420-01'])

# BIOL 3450-01: General Microbiology
# assigned to SET 524 at MWF1100+50
db.make_section('BIOL 3450-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jeremy W Bakelar', ['BIOL 3450-01'])

# BIOL 3455-01: General Microbiology Lab
# assigned to SET 304 at T0900+170
db.make_section('BIOL 3455-01', ['SET 304', '3 hour lab T0900'])
db.assign_faculty_sections('Jeremy W Bakelar', ['BIOL 3455-01'])

# BIOL 3455-02: General Microbiology Lab
# assigned to SET 304 at T1500+170
db.make_section('BIOL 3455-02', ['SET 304', '3 hour lab T0900'])
db.assign_faculty_sections('Jeremy W Bakelar', ['BIOL 3455-02'])

# BIOL 3460-01: Biology of Infectious Disease
# assigned to SET 201 at MW1500+75
db.make_section('BIOL 3460-01', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Donald H Warner', ['BIOL 3460-01'])

# BIOL 4040-01: Medical Ecology
# assigned to SET 501 at W0900+50
db.make_section('BIOL 4040-01', ['SET 501', '1 credit bell schedule'])
db.assign_faculty_sections('Marius Van der Merwe', ['BIOL 4040-01'])

# BIOL 4200-01: Plant Taxonomy (ALPP)
# assigned to SNOW 208 at TR1500+50
db.make_section('BIOL 4200-01', ['Science small lecture', '2 credit lecture'])
db.assign_faculty_sections('Del William Smith', ['BIOL 4200-01'])

# BIOL 4205-01: Plant Taxonomy Lab (ALPP)
# assigned to SNOW 208 at TR1600+170
db.make_section('BIOL 4205-01', ['Science small lecture', 'TR1600+170'])
db.assign_faculty_sections('Del William Smith', ['BIOL 4205-01'])

# BIOL 4280-01: Marine Biology
# assigned to SET 318 at MWF0900+50
db.make_section('BIOL 4280-01', ['SET 318', '3 credit bell schedule'])
db.assign_faculty_sections('Jennifer L Ciaccio', ['BIOL 4280-01'])

# BIOL 4300-01: Molecular Biology
# assigned to SET 216 at MWF0900+50
db.make_section('BIOL 4300-01', ['SET 216', '3 credit bell schedule'])
db.assign_faculty_sections('Martina Gaspari', ['BIOL 4300-01'])

# BIOL 4305-01: Molecular Biology Laboratory
# assigned to SET 308 at R0800+170
db.make_section('BIOL 4305-01', ['SET 308', '3 hour lab R0800'])
db.assign_faculty_sections('Martina Gaspari', ['BIOL 4305-01'])

# BIOL 4310-01: Advanced Bioinformatics
# assigned to SET 501 at TR1330+75
db.make_section('BIOL 4310-01', ['SET 501', '3 credit bell schedule'])
db.assign_faculty_sections('Randy Klabacka', ['BIOL 4310-01'])

# BIOL 4350-01: Animal Behavior
# assigned to SET 319 at TR1200+75
db.make_section('BIOL 4350-01', ['SET 319', '3 credit bell schedule'])
db.assign_faculty_sections('Curtis B Walker', ['BIOL 4350-01'])

# BIOL 4355-01: Animal Behavior Lab
# assigned to SET 319 at T1400+170
db.make_section('BIOL 4355-01', ['SET 319', '3 hour lab T0800'])
db.assign_faculty_sections('Curtis B Walker', ['BIOL 4355-01'])

# BIOL 4440-01: General Entomology
# assigned to SNOW 208 at TR1030+75
db.make_section('BIOL 4440-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Bryan K Stevens', ['BIOL 4440-01'])

# BIOL 4600-01: Plant Physiology
# assigned to SET 216 at MW1200+75
db.make_section('BIOL 4600-01', ['SET 216', '3 credit bell schedule'])
db.assign_faculty_sections("Erin E O'Brien", ['BIOL 4600-01'])

# BIOL 4605-01: Plant Physiology Lab
# assigned to SET 216 at W1500+170
db.make_section('BIOL 4605-01', ['SET 216', '3 hour lab W0900'])
db.assign_faculty_sections("Erin E O'Brien", ['BIOL 4605-01'])

# BIOL 4810R-01B: Independent Research
# assigned to SET 303 at M1400+180
db.make_section('BIOL 4810R-01B', ['SET 303', 'M1400+180'])

# BIOL 4890R-50: Life Science Internship
# assigned to SET 501 at W1715+110
db.make_section('BIOL 4890R-50', ['SET 501', 'W1715+110'])

# BIOL 4890R-51: Life Science Internship
# assigned to SET 501 at R1715+110
db.make_section('BIOL 4890R-51', ['SET 501', 'R1715+110'])

# BIOL 4910-01: Senior Seminar
# assigned to SET 501 at M0800+50
db.make_section('BIOL 4910-01', ['SET 501', '1 credit bell schedule'])

# BIOL 4910-02: Senior Seminar
# assigned to SET 501 at R1100+50
db.make_section('BIOL 4910-02', ['SET 501', '1 credit bell schedule'])

# BIOL 4910-03: Senior Seminar
# assigned to SET 501 at T1030+50
db.make_section('BIOL 4910-03', ['SET 501', '1 credit extended bell schedule'])

# BIOL 4990R-02: Seminar in Biology: Dental
# assigned to SET 303 at R1600+170
db.make_section('BIOL 4990R-02', ['SET 303', 'R1600+170'])
db.assign_faculty_sections('Scott E Bulloch', ['BIOL 4990R-02'])

# BIOL 4990R-50: Seminar in Biology
# assigned to SET 216 at W1800+50
db.make_section('BIOL 4990R-50', ['SET 216', '1 credit evening'])

# BTEC 1010-01: Fundamentals of Biotechnology
# assigned to SET 310 at TR1200+75
db.make_section('BTEC 1010-01', ['SET 310', '3 credit bell schedule'])
db.assign_faculty_sections('Douglas J Sainsbury', ['BTEC 1010-01'])

# BTEC 2020-01: Protein Purification and Analysis
# assigned to SET 304 at TR1300+110
db.make_section('BTEC 2020-01', ['SET 304', '4 hour lab TR0900'])
db.assign_faculty_sections('Jeremy W Bakelar', ['BTEC 2020-01'])

# BTEC 2030-01: Cell Culture Techniques
# assigned to SET 308 at MR1100+110
db.make_section('BTEC 2030-01', ['SET 308', 'MR1100+110'])
db.assign_faculty_sections('Martina Gaspari', ['BTEC 2030-01'])

# BTEC 2050-01: Zebrafish Maintenance & Method
# assigned to SET 303 at T1300+110
db.make_section('BTEC 2050-01', ['SET 303', '2 hour lab T0900'])
db.assign_faculty_sections('Hung Yu Shih', ['BTEC 2050-01'])

# BTEC 2050-01-alt: Zebrafish Maintenance & Method
# assigned to SET 303 at T1500+50
db.make_section('BTEC 2050-01-lab', ['SET 303', '1 credit extended bell schedule'])
db.assign_faculty_sections('Hung Yu Shih', ['BTEC 2050-01-lab'])

# BTEC 2050-02: Zebrafish Maintenance & Method
# assigned to SET 303 at T1300+110
db.make_section('BTEC 2050-02', []) # 'SET 303', '2 hour lab T0900'
# ./edit assign-faculty-sections 'Hung Yu Shih', 'BTEC 2050-02'
# cross-listed with BTEC 2050-01

# BTEC 2050-02-alt: Zebrafish Maintenance & Method
# assigned to SET 303 at T1600+50
db.make_section('BTEC 2050-02-lab', ['SET 303', '1 credit extended bell schedule'])
db.assign_faculty_sections('Hung Yu Shih', ['BTEC 2050-02-lab'])

# BTEC 3010-01: Sequencing Methods & Technique
# assigned to SET 312 at MW1530+75
db.make_section('BTEC 3010-01', ['SET 312', 'MW1530+75'])

# BTEC 4050-01A: In Situ Hybridization
# assigned to SET 303 at W1330+170
db.make_section('BTEC 4050-01A', ['SET 303', 'W1330+170'])
db.assign_faculty_sections('Hung Yu Shih', ['BTEC 4050-01A'])

# CHEM 1010-01: Introduction to Chemistry (PS)
# assigned to SNOW 113 at TR1030+75
db.make_section('CHEM 1010-01', ['Snow 113', 'Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Sarah Morgan Black', ['CHEM 1010-01'])

# CHEM 1010-02: Introduction to Chemistry (PS)
# assigned to SNOW 113 at TR1330+75
db.make_section('CHEM 1010-02', ['Snow 113', 'Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Sarah Morgan Black', ['CHEM 1010-02'])

# CHEM 1015-01: Introduction to Chemistry Lab (LAB)
# assigned to SET 405 at M0900+110
db.make_section('CHEM 1015-01', ['SET 405', '2 hour lab M0900'])

# CHEM 1015-02: Introduction to Chemistry Lab (LAB)
# assigned to SET 405 at M1100+110
db.make_section('CHEM 1015-02', ['SET 405', '2 hour lab M0900'])

# CHEM 1015-03: Introduction to Chemistry Lab (LAB)
# assigned to SET 405 at M1300+110
db.make_section('CHEM 1015-03', ['SET 405', '2 hour lab M0900'])

# CHEM 1120-01: Elem Organic / Bio Chemistry
# assigned to SNOW 216 at MTWR0900+50
db.make_section('CHEM 1120-01', ['Science small lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Jared M Hancock', ['CHEM 1120-01'])

# CHEM 1125-01: Elem Organic/Bio Chemistry Lab
# assigned to SET 404 at M1100+110
db.make_section('CHEM 1125-01', ['SET 404', '2 hour lab M0900'])
db.assign_faculty_sections('Jared M Hancock', ['CHEM 1125-01'])

# CHEM 1125-02: Elem Organic/Bio Chemistry Lab
# assigned to SET 404 at M1300+110
db.make_section('CHEM 1125-02', ['SET 404', '2 hour lab M0900'])

# CHEM 1150-01: Integrated Chemistry for Health Sciences (PS)
# assigned to SET 201 at MTWR0800+50
db.make_section('CHEM 1150-01', ['Science medium lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Jared M Hancock', ['CHEM 1150-01'])

# CHEM 1150-02: Integrated Chemistry for Health Sciences (PS)
# assigned to SET 201 at MTWR1400+50
db.make_section('CHEM 1150-02', ['Science medium lecture', '4 credit 4×50 extended bell schedule'])
db.assign_faculty_sections('Jared M Hancock', ['CHEM 1150-02'])

# CHEM 1150-03: Integrated Chemistry for Health Sciences (PS)
# assigned to SNOW 216 at MTWR1200+50
db.make_section('CHEM 1150-03', ['Science small lecture', '4 credit bell schedule'])

# CHEM 1155-01: Integrated Chemistry for Health Sciences Laboratory (LAB)
# assigned to SET 405 at T1000+170
db.make_section('CHEM 1155-01', ['SET 405', '3 hour lab T1000'])
db.assign_faculty_sections('Christina M Quinn', ['CHEM 1155-01'])

# CHEM 1155-02: Integrated Chemistry for Health Sciences Laboratory (LAB)
# assigned to SET 407 at W1000+170
db.make_section('CHEM 1155-02', ['SET 407', '3 hour lab W1000'])
db.assign_faculty_sections('Jared M Hancock', ['CHEM 1155-02'])

# CHEM 1155-03: Integrated Chemistry for Health Sciences Laboratory (LAB)
# assigned to SET 407 at W1300+170
db.make_section('CHEM 1155-03', ['SET 407', '3 hour lab W1000'])
db.assign_faculty_sections('Christina M Quinn', ['CHEM 1155-03'])

# CHEM 1155-05: Integrated Chemistry for Health Sciences Laboratory (LAB)
# assigned to SET 405 at T1600+170
db.make_section('CHEM 1155-05', ['SET 405', 'T1600+170'])
db.assign_faculty_sections('Paul H Shirley', ['CHEM 1155-05'])

# CHEM 1155-06: Integrated Chemistry for Health Sciences Laboratory (LAB)
# assigned to SET 405 at W0900+170
db.make_section('CHEM 1155-06', ['SET 405', '3 hour lab W0900'])
db.assign_faculty_sections('Teisha Richan', ['CHEM 1155-06'])

# CHEM 1155-50: Integrated Chemistry for Health Sciences Laboratory (LAB)
# assigned to SET 405 at T1900+170
db.make_section('CHEM 1155-50', ['SET 405', 'T1900+170'])
db.assign_faculty_sections('Paul H Shirley', ['CHEM 1155-50'])

# CHEM 1210-01: Principles of Chemistry I (PS)
# assigned to SET 201 at MTWR0900+50
db.make_section('CHEM 1210-01', ['Science medium lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Diana L Reese', ['CHEM 1210-01'])

# CHEM 1210-02: Principles of Chemistry I (PS)
# assigned to SET 201 at MTWR1000+50
db.make_section('CHEM 1210-02', ['Science medium lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Diana L Reese', ['CHEM 1210-02'])

# CHEM 1210-03: Principles of Chemistry I (PS)
# assigned to SNOW 216 at MTWR1300+50
db.make_section('CHEM 1210-03', ['Science small lecture', '4 credit 4×50 extended bell schedule'])

# CHEM 1215-01: Principles of Chemistry I Lab (LAB)
# assigned to SET 407 at T0700+170
db.make_section('CHEM 1215-01', ['SET 407', 'T0700+170'])
db.assign_faculty_sections('Christina M Quinn', ['CHEM 1215-01'])

# CHEM 1215-02: Principles of Chemistry I Lab (LAB)
# assigned to SET 409 at R1000+170
db.make_section('CHEM 1215-02', ['SET 409', '3 hour lab R1000'])
db.assign_faculty_sections('Christina M Quinn', ['CHEM 1215-02'])

# CHEM 1215-03: Principles of Chemistry I Lab (LAB)
# assigned to SET 407 at R1000+170
db.make_section('CHEM 1215-03', ['SET 407', '3 hour lab R1000'])

# CHEM 1215-04: Principles of Chemistry I Lab (LAB)
# assigned to SET 409 at R1300+170
db.make_section('CHEM 1215-04', ['SET 409', '3 hour lab R1000'])
db.assign_faculty_sections('Christina M Quinn', ['CHEM 1215-04'])

# CHEM 1215-05: Principles of Chemistry I Lab (LAB)
# assigned to SET 407 at R1600+170
db.make_section('CHEM 1215-05', ['SET 407', 'R1600+170'])
db.assign_faculty_sections('Megan R Liljenquist', ['CHEM 1215-05'])

# CHEM 1215-06: Principles of Chemistry I Lab (LAB)
# assigned to SET 409 at R1600+170
db.make_section('CHEM 1215-06', ['SET 409', 'R1600+170'])
db.assign_faculty_sections('Jacson Parker', ['CHEM 1215-06'])

# CHEM 1215-50: Principles of Chemistry I Lab (LAB)
# assigned to SET 409 at R1900+170
db.make_section('CHEM 1215-50', ['SET 409', 'R1900+170'])
db.assign_faculty_sections('David J Burr', ['CHEM 1215-50'])

# CHEM 1220-01: Principles of Chemistry II
# assigned to SET 420 at MTWR0800+50
db.make_section('CHEM 1220-01', ['Science small lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Gabriela Chilom', ['CHEM 1220-01'])

# CHEM 1220-02: Principles of Chemistry II
# assigned to SNOW 216 at MTWR1400+50
db.make_section('CHEM 1220-02', ['Science small lecture', '4 credit 4×50 extended bell schedule'])
db.assign_faculty_sections('Gabriela Chilom', ['CHEM 1220-02'])

# CHEM 1220-03: Principles of Chemistry II
# assigned to SET 420 at MTWR1000+50
db.make_section('CHEM 1220-03', ['Science small lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Wendy E Schatzberg', ['CHEM 1220-03'])

# CHEM 1225-01: Principles of Chemistry II Lab
# assigned to SET 409 at T0700+170
db.make_section('CHEM 1225-01', ['SET 409', 'T0700+170'])

# CHEM 1225-02: Principles of Chemistry II Lab
# assigned to SET 409 at T1000+170
db.make_section('CHEM 1225-02', ['SET 409', '3 hour lab T1000'])

# CHEM 1225-03: Principles of Chemistry II Lab
# assigned to SET 409 at T1300+170
db.make_section('CHEM 1225-03', ['SET 409', '3 hour lab T1000'])
db.assign_faculty_sections('Christina M Quinn', ['CHEM 1225-03'])

# CHEM 1225-04: Principles of Chemistry II Lab
# assigned to SET 407 at T1600+170
db.make_section('CHEM 1225-04', ['SET 407', 'T1600+170'])
db.assign_faculty_sections('David J Burr', ['CHEM 1225-04'])

# CHEM 1225-05: Principles of Chemistry II Lab
# assigned to SET 409 at T1600+170
db.make_section('CHEM 1225-05', ['SET 409', 'T1600+170'])
db.assign_faculty_sections('Jacson Parker', ['CHEM 1225-05'])

# CHEM 1225-50: Principles of Chemistry II Lab
# assigned to SET 407 at T1900+170
db.make_section('CHEM 1225-50', ['SET 407', 'T1900+170'])
db.assign_faculty_sections('David J Burr', ['CHEM 1225-50'])

# CHEM 2310-01: Organic Chemistry I
# assigned to SET 420 at MTWRF0900+50
db.make_section('CHEM 2310-01', ['Science small lecture', '5 credit bell schedule'])
db.assign_faculty_sections('Rico Del Sesto', ['CHEM 2310-01'])

# CHEM 2310-02: Organic Chemistry I
# assigned to SNOW 216 at MTWRF1100+50
db.make_section('CHEM 2310-02', ['Science small lecture', '5 credit bell schedule'])

# CHEM 2315-01: Organic Chemistry I Lab
# assigned to SET 404 at R1000+170
db.make_section('CHEM 2315-01', ['SET 404', '3 hour lab R1000'])
db.assign_faculty_sections('Teisha Richan', ['CHEM 2315-01'])

# CHEM 2315-02: Organic Chemistry I Lab
# assigned to SET 404 at R1300+170
db.make_section('CHEM 2315-02', ['SET 404', '3 hour lab R1000'])
db.assign_faculty_sections('Teisha Richan', ['CHEM 2315-02'])

# CHEM 2320-01: Organic Chemistry II
# assigned to SET 201 at MTWRF1100+50
db.make_section('CHEM 2320-01', ['Science medium lecture', '5 credit bell schedule'])
db.assign_faculty_sections('Rico Del Sesto', ['CHEM 2320-01'])

# CHEM 2320-02: Organic Chemistry II
# assigned to SET 420 at MTWRF1200+50
db.make_section('CHEM 2320-02', ['Science small lecture', '5 credit bell schedule'])
db.assign_faculty_sections('Diana L Reese', ['CHEM 2320-02'])

# CHEM 2325-01: Organic Chemistry II Lab
# assigned to SET 404 at T0900+170
db.make_section('CHEM 2325-01', ['SET 404', '3 hour lab T0900'])
db.assign_faculty_sections('Teisha Richan', ['CHEM 2325-01'])

# CHEM 2325-02: Organic Chemistry II Lab
# assigned to SET 404 at T1200+170
db.make_section('CHEM 2325-02', ['SET 404', '3 hour lab T0900'])
db.assign_faculty_sections('Teisha Richan', ['CHEM 2325-02'])

# CHEM 2325-03: Organic Chemistry II Lab
# assigned to SET 404 at T1500+170
db.make_section('CHEM 2325-03', ['SET 404', '3 hour lab T0900'])

# CHEM 2325-04: Organic Chemistry II Lab
# assigned to SET 404 at W0900+170
db.make_section('CHEM 2325-04', ['SET 404', '3 hour lab W0900'])

# CHEM 2325-05: Organic Chemistry II Lab
# assigned to SET 404 at W1200+170
db.make_section('CHEM 2325-05', ['SET 404', '3 hour lab W0900'])
db.assign_faculty_sections('Teisha Richan', ['CHEM 2325-05'])

# CHEM 2325-06: Organic Chemistry II Lab
# assigned to SET 404 at W1500+170
db.make_section('CHEM 2325-06', ['SET 404', '3 hour lab W0900'])
db.assign_faculty_sections('Megan R Liljenquist', ['CHEM 2325-06'])

# CHEM 2325-50: Organic Chemistry II Lab
# assigned to SET 404 at T1800+170
db.make_section('CHEM 2325-50', ['SET 404', 'T1800+170'])

# CHEM 3070-01: Physical Chemistry II
# assigned to SET 420 at MTWR1100+50
db.make_section('CHEM 3070-01', ['Science small lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Wendy E Schatzberg', ['CHEM 3070-01'])

# CHEM 3075-01: Physical Chemistry II Lab
# assigned to SNOW 103 at T1600+170
db.make_section('CHEM 3075-01', ['Snow 103', 'T1600+170'])
db.assign_faculty_sections('Wendy E Schatzberg', ['CHEM 3075-01'])

# CHEM 3300-01: Instrumental Analysis
# assigned to SNOW 216 at MWF1000+50
db.make_section('CHEM 3300-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Gabriela Chilom', ['CHEM 3300-01'])

# CHEM 3300-01-alt: Instrumental Analysis
# assigned to SNOW 103 at R1500+170
db.make_section('CHEM 3300-01-SI', ['Snow 103', '3 hour lab R0900'])

# CHEM 3510-01: Biochemistry I
# assigned to SET 420 at MW1330+75
db.make_section('CHEM 3510-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jennifer A Meyer', ['CHEM 3510-01'])

# CHEM 3515-01: Biochemistry I Lab
# assigned to SET 308 at R1300+170
db.make_section('CHEM 3515-01', ['SET 308', '3 hour lab R1000'])
db.assign_faculty_sections('Jennifer A Meyer', ['CHEM 3515-01'])

# CHEM 3515-02: Biochemistry I Lab
# assigned to SET 308 at R1600+170
db.make_section('CHEM 3515-02', ['SET 308', 'R1600+170'])
db.assign_faculty_sections('Cutler Cowdin', ['CHEM 3515-02'])

# CHEM 3520-01: Biochemistry II
# assigned to SET 201 at MW1200+75
db.make_section('CHEM 3520-01', ['Science medium lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jennifer A Meyer', ['CHEM 3520-01'])

# CHEM 3525-01: Biochemistry II Lab
# assigned to SET 308 at T1000+170
db.make_section('CHEM 3525-01', ['SET 308', '3 hour lab T1000'])

# CHEM 3525-02: Biochemistry II Lab
# assigned to SET 308 at T1300+170
db.make_section('CHEM 3525-02', ['SET 308', '3 hour lab T1000'])
db.assign_faculty_sections('Jennifer A Meyer', ['CHEM 3525-02'])

# CHEM 3525-03: Biochemistry II Lab
# assigned to SET 308 at T1600+170
db.make_section('CHEM 3525-03', ['SET 308', 'T1600+170'])
db.assign_faculty_sections('Cutler Cowdin', ['CHEM 3525-03'])

# CHEM 4800R-01: Independent Research
# assigned to SNOW 204 at MTWRF1000+50
db.make_section('CHEM 4800R-01', ['Snow 204', '5 credit bell schedule'])
db.assign_faculty_sections('Rico Del Sesto', ['CHEM 4800R-01'])

# CHEM 4800R-02: Independent Research
# assigned to SNOW 204 at MTWRF1200+50
db.make_section('CHEM 4800R-02', ['Snow 204', '5 credit bell schedule'])
db.assign_faculty_sections('Wendy E Schatzberg', ['CHEM 4800R-02'])

# CHEM 4800R-03: Independent Research
# assigned to SNOW 204 at MTWRF1100+50
db.make_section('CHEM 4800R-03', ['Snow 204', '5 credit bell schedule'])

# CHEM 4800R-04: Independent Research
# assigned to SNOW 204 at MTWRF1500+50
db.make_section('CHEM 4800R-04', ['Snow 204', '5 credit extended bell schedule'])
db.assign_faculty_sections('Gabriela Chilom', ['CHEM 4800R-04'])

# CHEM 4800R-06: Independent Research
# assigned to SNOW 204 at MTWRF1600+50
db.make_section('CHEM 4800R-06', ['Snow 204', '5 credit extended bell schedule'])
db.assign_faculty_sections('Diana L Reese', ['CHEM 4800R-06'])

# CHEM 4910-01: Chemistry Senior Seminar
# assigned to SET 201 at F1200+50
db.make_section('CHEM 4910-01', ['Science medium lecture', '1 credit extended bell schedule'])

# ECE 2100-01: Semiconductor Devices
# assigned to SET 102 at MW1200+75
db.make_section('ECE 2100-01', ['SET 102', '3 credit bell schedule'])
db.assign_faculty_sections('Andrew Gregory Toth', ['ECE 2100-01'])

# ECE 2280-01: Microelectronics
# assigned to SET 102 at MWF1100+50
db.make_section('ECE 2280-01', ['SET 102', '3 credit bell schedule'])
db.assign_faculty_sections('Sai C Radavaram', ['ECE 2280-01'])

# ECE 2285-01: Microelectronics Lab
# assigned to SET 101 at T0800+110
db.make_section('ECE 2285-01', ['SET 101', '2 hour lab T0800'])
db.assign_faculty_sections('Sai C Radavaram', ['ECE 2285-01'])

# ECE 3500-01: Signals and Systems
# assigned to SET 523 at MW1500+75
db.make_section('ECE 3500-01', ['SET 523', '3 credit bell schedule'])
db.assign_faculty_sections('Kameron J Eves', ['ECE 3500-01'])

# ECE 3600-01: Power Electronics
# assigned to SET 523 at MW1330+75
db.make_section('ECE 3600-01', ['SET 523', '3 credit bell schedule'])
db.assign_faculty_sections('Sai C Radavaram', ['ECE 3600-01'])

# ECE 3605-01: Power Electronics Lab
# assigned to SET 101 at T1200+110
db.make_section('ECE 3605-01', ['SET 101', '2 hour lab T0800'])
db.assign_faculty_sections('David Brent Christensen', ['ECE 3605-01'])

# ECE 4010-01: EE Product Design II
# assigned to SET 219 at MWF1330+180
db.make_section('ECE 4010-01', ['SET 219', 'MWF1330+180'])
db.assign_faculty_sections('Brant A Ross', ['ECE 4010-01'])

# ECE 4510-01: Image Processing
# assigned to SET 523 at TR0900+75
db.make_section('ECE 4510-01', ['SET 523', '3 credit bell schedule'])
db.assign_faculty_sections('Jeffrey Anderson', ['ECE 4510-01'])

# ECE 4730-01: Embedded Systems II
# assigned to SET 523 at MW1630+75
db.make_section('ECE 4730-01', ['SET 523', '3 credit bell schedule'])
db.assign_faculty_sections('Jeffrey Anderson', ['ECE 4730-01'])

# ECE 4735-01: Embedded Systems II Lab
# assigned to SET 101 at T1400+110
db.make_section('ECE 4735-01', ['SET 101', '2 hour lab T0800'])
db.assign_faculty_sections('Jeffrey Anderson', ['ECE 4735-01'])

# ECE 4990-01: Special Topics: Human-Machine Interfacing
# assigned to SET 101 at F1000+110
db.make_section('ECE 4990-01-lab', ['SET 101', 'F1000+110'])
db.assign_faculty_sections('Bing Jiang', ['ECE 4990-01-lab'])

# ECE 4990-01-alt: Special Topics: Human-Machine Interfacing
# assigned to SET 523 at MW1200+75
db.make_section('ECE 4990-01', ['SET 523', '3 credit bell schedule'])
db.assign_faculty_sections('Bing Jiang', ['ECE 4990-01'])

# ECE 4990-02: Special Topics: Autopilot
# assigned to SET 523 at TR1030+75
db.make_section('ECE 4990-02', ['SET 523', '3 credit bell schedule'])
db.assign_faculty_sections('Kameron J Eves', ['ECE 4990-02'])

# ECE 4990-03: Special Topics: Antenna Engineering
# assigned to SET 101 at F0800+115
db.make_section('ECE 4990-03-lab', ['SET 101', 'F0800+115'])
db.assign_faculty_sections('Sai C Radavaram', ['ECE 4990-03-lab'])

# ECE 4990-03-alt: Special Topics: Antenna Engineering
# assigned to SET 523 at TR1630+75
db.make_section('ECE 4990-03', ['SET 523', '3 credit bell schedule'])
db.assign_faculty_sections('Sai C Radavaram', ['ECE 4990-03'])

# ENVS 1010-01: Intro to Environmental Science (PS)
# assigned to SET 524 at TR1200+75
db.make_section('ENVS 1010-01', ['Science small lecture', '3 credit bell schedule'])

# ENVS 1010-03: Intro to Environmental Science (PS)
# assigned to SET 524 at TR1330+75
db.make_section('ENVS 1010-03', ['Science small lecture', '3 credit bell schedule'])

# ENVS 1010-04: Intro to Environmental Science (PS)
# assigned to SET 524 at MW1330+75
db.make_section('ENVS 1010-04', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Greg L Melton', ['ENVS 1010-04'])

# ENVS 1010-05: Intro to Environmental Science (PS)
# assigned to SNOW 113 at TR1500+75
db.make_section('ENVS 1010-05', ['Snow 113', 'Science small lecture', '3 credit bell schedule'])

# ENVS 1010-06: Intro to Environmental Science (PS)
# assigned to SNOW 113 at MW1330+75
db.make_section('ENVS 1010-06', ['Snow 113', 'Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Marshall Topham', ['ENVS 1010-06'])

# ENVS 1010-07: Intro to Environmental Science (PS)
# assigned to SNOW 128 at TR1330+75
db.make_section('ENVS 1010-07', ['Science small lecture', '3 credit bell schedule'])

# ENVS 1099-01: Recitation for Majors
# assigned to SET 526 at F1000+50
db.make_section('ENVS 1099-01', ['SET 526', '1 credit bell schedule'])
db.assign_faculty_sections('Christina Pondell', ['ENVS 1099-01'])

# ENVS 1210-01: Introduction to Environmental Science
# assigned to SNOW 113 at TR1200+75
db.make_section('ENVS 1210-01', ['Snow 113', 'Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Marzieh Ghasemi', ['ENVS 1210-01'])

# ENVS 1215-01: Introduction to Environmental Science Laboratory
# assigned to SET 526 at M1300+170
db.make_section('ENVS 1215-01', ['SET 526', '3 hour lab M1000'])
db.assign_faculty_sections('Christina Pondell', ['ENVS 1215-01'])

# ENVS 1215-02: Introduction to Environmental Science Laboratory
# assigned to SET 526 at R1330+165
db.make_section('ENVS 1215-02', ['SET 526', 'R1330+165'])
db.assign_faculty_sections('Christina Pondell', ['ENVS 1215-02'])

# ENVS 2099R-50: Special Topics in Environmental Science: The Geology of Foundation Engineering in Southern Utah
# assigned to SET 526 at TR1800+75
db.make_section('ENVS 2099R-50', ['SET 526', 'TR1800+75'])
db.assign_faculty_sections('Hugo Elio Angeles', ['ENVS 2099R-50'])

# ENVS 2210-01: Environmental Pollution and Remediation Techniques
# assigned to SNOW 128 at MW1200+75
db.make_section('ENVS 2210-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Marzieh Ghasemi', ['ENVS 2210-01'])

# ENVS 2700R-01: Field Methods EnvSci
# assigned to SET 527 at F1400+170
db.make_section('ENVS 2700R-01', ['SET 527', 'F1400+170'])
db.assign_faculty_sections('Alexander R Tye', ['ENVS 2700R-01'])

# ENVS 3110-01: Scientific Writing
# assigned to SET 408 at MWF1100+50
db.make_section('ENVS 3110-01', ['SET 408', '3 credit bell schedule'])
db.assign_faculty_sections('Jerald D Harris', ['ENVS 3110-01'])

# ENVS 3210-01: Soil Science
# assigned to SET 526 at TR0900+75
db.make_section('ENVS 3210-01', ['SET 526', '3 credit bell schedule'])
db.assign_faculty_sections('Christina Pondell', ['ENVS 3210-01'])

# ENVS 3280-50: Environmental Law
# assigned to SNOW 128 at TR1800+110
db.make_section('ENVS 3280-50', ['Science small lecture', 'TR1800+110'])

# ENVS 3410-01: Air Quality and Control
# assigned to SET 522 at MWF1000+50
db.make_section('ENVS 3410-01', ['SET 522', '3 credit bell schedule'])
db.assign_faculty_sections('Marzieh Ghasemi', ['ENVS 3410-01'])

# ENVS 3920-50: Peruvian Amazon Natural Histor
# assigned to SNOW 113 at W1800+50
db.make_section('ENVS 3920-50', ['Snow 113', 'Science small lecture', '1 credit evening'])
db.assign_faculty_sections('Marius Van der Merwe', ['ENVS 3920-50'])

# ENVS 4910-01: Senior Seminar
# assigned to SET 408 at F1200+50
db.make_section('ENVS 4910-01', ['SET 408', '1 credit extended bell schedule'])

# GEO 1010-01: Introduction to Geology (PS)
# assigned to SET 524 at TR0900+75
db.make_section('GEO 1010-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Greg L Melton', ['GEO 1010-01'])

# GEO 1010-50: Introduction to Geology (PS)
# assigned to SNOW 128 at MW1800+75
db.make_section('GEO 1010-50', ['Science small lecture', 'MW1800+75'])

# GEO 1015-01: Introduction to Geology Lab (LAB)
# assigned to SET 527 at W0900+110
db.make_section('GEO 1015-01', ['SET 527', '2 hour lab W0900'])
db.assign_faculty_sections('Greg L Melton', ['GEO 1015-01'])

# GEO 1015-03: Introduction to Geology Lab (LAB)
# assigned to SET 527 at T1100+110
db.make_section('GEO 1015-03', ['SET 527', '2 hour lab T0900'])

# GEO 1015-04: Introduction to Geology Lab (LAB)
# assigned to SET 527 at T1500+110
db.make_section('GEO 1015-04', ['SET 527', '2 hour lab T0900'])

# GEO 1015-50: Introduction to Geology Lab (LAB)
# assigned to SET 527 at T1700+110
db.make_section('GEO 1015-50', ['SET 527', 'T1700+110'])
db.assign_faculty_sections('David R Black', ['GEO 1015-50'])

# GEO 1015-51: Introduction to Geology Lab (LAB)
# assigned to SET 527 at T1900+110
db.make_section('GEO 1015-51', ['SET 527', 'T1900+110'])

# GEO 1050-01: Geology of the National Parks (PS)
# assigned to SET 527 at W1100+110
db.make_section('GEO 1050-01', ['SET 527', '2 hour lab W0900'])

# GEO 1110-01: Physical Geology (PS)
# assigned to SET 522 at TR0900+75
db.make_section('GEO 1110-01', ['SET 522', '3 credit bell schedule'])
db.assign_faculty_sections('Janice M Hayden', ['GEO 1110-01'])

# GEO 1115-01: Physical Geology Lab
# assigned to SET 522 at W1100+170
db.make_section('GEO 1115-01', ['SET 522', '3 hour lab W0800'])
db.assign_faculty_sections('Janice M Hayden', ['GEO 1115-01'])

# GEO 1220-01: Historical Geology
# assigned to SET 522 at TR1030+75
db.make_section('GEO 1220-01', ['SET 522', '3 credit bell schedule'])
db.assign_faculty_sections('Jerald D Harris', ['GEO 1220-01'])

# GEO 1225-01: Historical Geology Lab
# assigned to SET 522 at R1630+170
db.make_section('GEO 1225-01', ['SET 522', 'R1630+170'])
db.assign_faculty_sections('Jerald D Harris', ['GEO 1225-01'])

# GEO 2700R-01: Field Methods in Geoscience Research
# assigned to SET 527 at F1400+170
db.make_section('GEO 2700R-01', []) # 'SET 527', 'F1400+170'
# ./edit assign-faculty-sections 'Alexander R Tye', 'GEO 2700R-01'
# cross-listed with ENVS 2700R-01

# GEO 3110-01: Scientific Writing
# assigned to SET 408 at MWF1100+50
db.make_section('GEO 3110-01', []) # 'SET 408', '3 credit bell schedule'
# ./edit assign-faculty-sections 'Jerald D Harris', 'GEO 3110-01'
# cross-listed with ENVS 3110-01

# GEO 3500-01: Geomorphology
# assigned to SET 408 at R1200+170
db.make_section('GEO 3500-01-lab', ['SET 408', '3 hour lab R0900'])
db.assign_faculty_sections('Alexander R Tye', ['GEO 3500-01-lab'])

# GEO 3500-01-alt: Geomorphology
# assigned to SET 408 at TR1500+75
db.make_section('GEO 3500-01', ['SET 408', '3 credit bell schedule'])
db.assign_faculty_sections('Alexander R Tye', ['GEO 3500-01'])

# GEO 3600-01: Ig/Met Petrology
# assigned to SET 522 at MW1500+75
db.make_section('GEO 3600-01', ['SET 522', '3 credit bell schedule'])
db.assign_faculty_sections('Greg L Melton', ['GEO 3600-01'])

# GEO 3600-01-alt: Ig/Met Petrology
# assigned to SET 522 at T1200+170
db.make_section('GEO 3600-01-lab', ['SET 522', '3 hour lab T0900'])
db.assign_faculty_sections('Greg L Melton', ['GEO 3600-01-lab'])

# GEO 3710-01: Hydrology
# assigned to SET 524 at TR1500+75
db.make_section('GEO 3710-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Marzieh Ghasemi', ['GEO 3710-01'])

# GEO 4000R-01: Selected Geology Field Excursions
# assigned to SET 527 at F1100+50
db.make_section('GEO 4000R-01', ['SET 527', '1 credit bell schedule'])

# GEO 4910-01: Senior Seminar
# assigned to SNOW 216 at F1200+50
db.make_section('GEO 4910-01', ['Science small lecture', '1 credit extended bell schedule'])

# GEOG 1000-01: Physical Geography: Supplemental Instruction (PS)
# assigned to SET 524 at MWF1000+50
db.make_section('GEOG 1000-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jerald D Harris', ['GEOG 1000-01'])

# GEOG 1000-01-alt: Physical Geography: Supplemental Instruction (PS)
# assigned to SNOW 216 at R1000+50
db.make_section('GEOG 1000-01-SI', ['Science small lecture', '1 credit bell schedule'])

# GEOG 1000-02: Physical Geography (PS)
# assigned to SET 524 at MW1200+75
db.make_section('GEOG 1000-02', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Zhenyu Jin', ['GEOG 1000-02'])

# GEOG 1000-03: Physical Geography (PS)
# assigned to SNOW 113 at TR0900+75
db.make_section('GEOG 1000-03', ['Snow 113', 'Science small lecture', '3 credit bell schedule'])

# GEOG 1005-01: Physical Geography Lab (LAB)
# assigned to SET 526 at T1100+110
db.make_section('GEOG 1005-01', ['SET 526', '2 hour lab T0900'])
db.assign_faculty_sections('Christina Pondell', ['GEOG 1005-01'])

# GEOG 1005-02: Physical Geography Lab (LAB)
# assigned to SET 526 at T1300+110
db.make_section('GEOG 1005-02', ['SET 526', '2 hour lab T0900'])
db.assign_faculty_sections('Christina Pondell', ['GEOG 1005-02'])

# GEOG 1005-03: Physical Geography Lab (LAB)
# assigned to SET 526 at W0900+110
db.make_section('GEOG 1005-03', ['SET 526', '2 hour lab W0900'])
db.assign_faculty_sections('Zhenyu Jin', ['GEOG 1005-03'])

# GEOG 1005-04: Physical Geography Lab (LAB)
# assigned to SET 526 at W1100+110
db.make_section('GEOG 1005-04', ['SET 526', '2 hour lab W0900'])

# GEOG 1005-05: Physical Geography Lab (LAB)
# assigned to SET 526 at R1100+110
db.make_section('GEOG 1005-05', ['SET 526', '2 hour lab R0900'])

# GEOG 3600-01: Introduction to Geographic Information Systems
# assigned to SET 408 at TR1030+75
db.make_section('GEOG 3600-01', ['SET 408', '3 credit bell schedule'])
db.assign_faculty_sections('Zhenyu Jin', ['GEOG 3600-01'])

# GEOG 3605-01: Introduction to Geographic Information Systems Laboratory
# assigned to SET 408 at T1200+170
db.make_section('GEOG 3605-01', ['SET 408', '3 hour lab T0900'])
db.assign_faculty_sections('Zhenyu Jin', ['GEOG 3605-01'])

# GEOG 4180-01: Geoprocessing with Python
# assigned to SET 408 at MW1330+75
db.make_section('GEOG 4180-01', ['SET 408', '3 credit bell schedule'])
db.assign_faculty_sections('Zhenyu Jin', ['GEOG 4180-01'])

# MATH 1010-03: Intermediate Algebra
# assigned to SNOW 3 at MTWR1100+50
db.make_section('MATH 1010-03', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Violeta Adina Ionita', ['MATH 1010-03'])

# MATH 1010-04: Intermediate Algebra
# assigned to SNOW 145 at MW1300+100
db.make_section('MATH 1010-04', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Elizabeth Karen Ludlow', ['MATH 1010-04'])

# MATH 1010-05: Intermediate Algebra
# assigned to SNOW 145 at TR1500+100
db.make_section('MATH 1010-05', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Odean Bowler', ['MATH 1010-05'])

# MATH 1010-06: Intermediate Algebra
# assigned to SNOW 145 at MW1500+100
db.make_section('MATH 1010-06', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Odean Bowler', ['MATH 1010-06'])

# MATH 1010-07: Intermediate Algebra
# assigned to SNOW 3 at MTWR1200+50
db.make_section('MATH 1010-07', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Violeta Adina Ionita', ['MATH 1010-07'])

# MATH 1010-50: Intermediate Algebra
# assigned to SNOW 147 at TR1800+100
db.make_section('MATH 1010-50', ['Math lecture', 'TR1800+100'])

# MATH 1030-01: Quantitative Reasoning (MA)
# assigned to SNOW 125 at MW1500+75
db.make_section('MATH 1030-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Elizabeth Karen Ludlow', ['MATH 1030-01'])

# MATH 1030-02: Quantitative Reasoning (MA)
# assigned to SNOW 124 at TR0730+75
db.make_section('MATH 1030-02', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Craig D Seegmiller', ['MATH 1030-02'])

# MATH 1030-03: Quantitative Reasoning (MA)
# assigned to SNOW 124 at TR0900+75
db.make_section('MATH 1030-03', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Craig D Seegmiller', ['MATH 1030-03'])

# MATH 1030-04: Quantitative Reasoning (MA)
# assigned to SNOW 125 at MW1330+75
db.make_section('MATH 1030-04', ['Math lecture', '3 credit bell schedule'])

# MATH 1030-05: Quantitative Reasoning (MA)
# assigned to SNOW 150 at TR1200+75
db.make_section('MATH 1030-05', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jeffrey P Harrah', ['MATH 1030-05'])

# MATH 1030-06: Quantitative Reasoning (MA)
# assigned to SNOW 150 at TR1330+75
db.make_section('MATH 1030-06', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jeffrey P Harrah', ['MATH 1030-06'])

# MATH 1040-01: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF0800+50
db.make_section('MATH 1040-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('James P Fitzgerald', ['MATH 1040-01'])

# MATH 1040-02: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF0900+50
db.make_section('MATH 1040-02', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('James P Fitzgerald', ['MATH 1040-02'])

# MATH 1040-03: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF1000+50
db.make_section('MATH 1040-03', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('James P Fitzgerald', ['MATH 1040-03'])

# MATH 1040-04: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF1200+50
db.make_section('MATH 1040-04', ['Math lecture', 'MWF1200+50'])

# MATH 1040-05: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF1100+50
db.make_section('MATH 1040-05', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Tye K Rogers', ['MATH 1040-05'])

# MATH 1040-06: Introduction to Statistics (MA)
# assigned to SNOW 125 at TR1330+75
db.make_section('MATH 1040-06', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Tye K Rogers', ['MATH 1040-06'])

# MATH 1040-07: Introduction to Statistics (MA)
# assigned to SNOW 151 at TR1200+75
db.make_section('MATH 1040-07', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jameson C Hardy', ['MATH 1040-07'])

# MATH 1040-08: Introduction to Statistics (MA)
# assigned to SNOW 124 at MW1500+75
db.make_section('MATH 1040-08', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Paula Manuele Temple', ['MATH 1040-08'])

# MATH 1040-09: Introduction to Statistics (MA)
# assigned to SNOW 150 at MW1200+75
db.make_section('MATH 1040-09', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jameson C Hardy', ['MATH 1040-09'])

# MATH 1040-10: Introduction to Statistics (MA)
# assigned to SNOW 124 at TR1200+75
db.make_section('MATH 1040-10', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jie Liu', ['MATH 1040-10'])

# MATH 1040-11: Introduction to Statistics (MA)
# assigned to SNOW 124 at TR1630+75
db.make_section('MATH 1040-11', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Ryan C McConnell', ['MATH 1040-11'])

# MATH 1040-12: Introduction to Statistics (MA)
# assigned to SNOW 125 at TR1630+75
db.make_section('MATH 1040-12', ['Math lecture', '3 credit bell schedule'])

# MATH 1040-14: Introduction to Statistics (MA)
# assigned to SNOW 124 at MW1630+75
db.make_section('MATH 1040-14', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Robert T Reimer', ['MATH 1040-14'])

# MATH 1050-01: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 3 at MTWR0800+50
db.make_section('MATH 1050-01', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Violeta Adina Ionita', ['MATH 1050-01'])

# MATH 1050-02: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 3 at MTWR0900+50
db.make_section('MATH 1050-02', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Violeta Adina Ionita', ['MATH 1050-02'])

# MATH 1050-03: College Algebra / Pre-Calculus: Supplemental Instruction (MA)
# assigned to SNOW 125 at F1100+50
db.make_section('MATH 1050-03', ['Math lecture', 'F1100+50'])

# MATH 1050-03-alt: College Algebra / Pre-Calculus: Supplemental Instruction (MA)
# assigned to SNOW 125 at MTWR1100+50
db.make_section('MATH 1050-03-alt', ['Math lecture', 'MTWR1100+50'])
db.assign_faculty_sections('Costel Ionita', ['MATH 1050-03-alt'])

# MATH 1050-04: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 147 at MTWR1200+50
db.make_section('MATH 1050-04', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Clare C Banks', ['MATH 1050-04'])

# MATH 1050-05: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 145 at TR1300+100
db.make_section('MATH 1050-05', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Dawn Lashell Kidd-Thomas', ['MATH 1050-05'])

# MATH 1050-06: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 112 at MTWR1200+50
db.make_section('MATH 1050-06', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Craig D Seegmiller', ['MATH 1050-06'])

# MATH 1060-01: Trigonometry (MA)
# assigned to SNOW 147 at TR0900+75
db.make_section('MATH 1060-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Ross C Decker', ['MATH 1060-01'])

# MATH 1060-02: Trigonometry (MA)
# assigned to SNOW 147 at TR1030+75
db.make_section('MATH 1060-02', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Ross C Decker', ['MATH 1060-02'])

# MATH 1080-01: Pre-Calculus with Trigonometry (MA)
# assigned to SNOW 145 at MTWRF1000+50
db.make_section('MATH 1080-01', ['Math lecture', '5 credit bell schedule'])
db.assign_faculty_sections('Jameson C Hardy', ['MATH 1080-01'])

# MATH 1100-02: Business Calculus (MA)
# assigned to SNOW 124 at MW1330+75
db.make_section('MATH 1100-02', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Trevor K Johnson', ['MATH 1100-02'])

# MATH 1210-01: Calculus I (MA)
# assigned to SNOW 145 at MTWR1200+50
db.make_section('MATH 1210-01', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Trevor K Johnson', ['MATH 1210-01'])

# MATH 1210-02: Calculus I (MA)
# assigned to SNOW 125 at MTWR0800+50
db.make_section('MATH 1210-02', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Costel Ionita', ['MATH 1210-02'])

# MATH 1210-03: Calculus I (MA)
# assigned to SNOW 145 at MTWR1100+50
db.make_section('MATH 1210-03', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Bhuvaneswari Sambandham', ['MATH 1210-03'])

# MATH 1220-01: Calculus II (MA)
# assigned to SNOW 147 at MTWR0800+50
db.make_section('MATH 1220-01', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Clare C Banks', ['MATH 1220-01'])

# MATH 1220-02: Calculus II (MA)
# assigned to SNOW 125 at MTWR0900+50
db.make_section('MATH 1220-02', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Costel Ionita', ['MATH 1220-02'])

# MATH 2010-01: Math for Elementary Teachers I
# assigned to SNOW 150 at T1630+150
db.make_section('MATH 2010-01', ['Math lecture', 'T1630+150'])
db.assign_faculty_sections('Jeffrey P Harrah', ['MATH 2010-01'])

# MATH 2020-01: Math for Elemen Teachers II
# assigned to SNOW 150 at TR1030+75
db.make_section('MATH 2020-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jeffrey P Harrah', ['MATH 2020-01'])

# MATH 2020-02: Math for Elemen Teachers II
# assigned to SNOW 150 at W1630+150
db.make_section('MATH 2020-02', ['Math lecture', 'W1630+150'])
db.assign_faculty_sections('Jeffrey P Harrah', ['MATH 2020-02'])

# MATH 2200-01: Discrete Mathematics
# assigned to SNOW 112 at TR1030+75
db.make_section('MATH 2200-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Steven McKay Sullivan', ['MATH 2200-01'])

# MATH 2210-01: Multivariable Calculus (MA)
# assigned to SNOW 112 at MTWR0900+50
db.make_section('MATH 2210-01', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Steven McKay Sullivan', ['MATH 2210-01'])

# MATH 2250-01: Differential Equations and Linear Algebra
# assigned to SNOW 125 at MTWF1000+50
db.make_section('MATH 2250-01', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Bhuvaneswari Sambandham', ['MATH 2250-01'])

# MATH 2270-01: Linear Algebra
# assigned to SNOW 151 at TR0900+75
db.make_section('MATH 2270-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Md Sazib Hasan', ['MATH 2270-01'])

# MATH 2280-01: Ordinary Differential Equations
# assigned to SNOW 151 at MW1200+75
db.make_section('MATH 2280-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Bhuvaneswari Sambandham', ['MATH 2280-01'])

# MATH 3050-01: Stochastic Modeling and Applications
# assigned to SNOW 151 at TR1030+75
db.make_section('MATH 3050-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Md Sazib Hasan', ['MATH 3050-01'])

# MATH 3200-01: Introduction to Analysis I
# assigned to SNOW 125 at TR1200+75
db.make_section('MATH 3200-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Costel Ionita', ['MATH 3200-01'])

# MATH 3450-01: Statistical Inference
# assigned to SNOW 124 at TR1030+75
db.make_section('MATH 3450-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jie Liu', ['MATH 3450-01'])

# MATH 3900-01: Number Theory
# assigned to SNOW 112 at MWF1000+50
db.make_section('MATH 3900-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Steven McKay Sullivan', ['MATH 3900-01'])

# MATH 4250-01: Programming for Scientific Computation
# assigned to SNOW 147 at MW1500+100
db.make_section('MATH 4250-01', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Vinodh Kumar Chellamuthu', ['MATH 4250-01'])

# MATH 4400-01: Financial Mathematics
# assigned to SNOW 124 at TR1330+75
db.make_section('MATH 4400-01', ['Math lecture', '3 credit bell schedule'])
db.assign_faculty_sections('Jie Liu', ['MATH 4400-01'])

# MATH 4410-01: Actuarial Exam FM/ 2 Preparation
# assigned to SNOW 124 at T1500+75
db.make_section('MATH 4410-01', ['Math lecture', 'T1500+75'])
db.assign_faculty_sections('Jie Liu', ['MATH 4410-01'])

# MATH 4800-01: Industrial Careers in Mathematics
# assigned to SNOW 147 at MW1645+75
db.make_section('MATH 4800-01', ['Math lecture', 'MW1645+75'])
db.assign_faculty_sections('Vinodh Kumar Chellamuthu', ['MATH 4800-01'])

# MATH 0900-01: Transitional Math I
# assigned to SNOW 144 at MTWR1200+50
db.make_section('MATH 0900-01', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Paula Manuele Temple', ['MATH 0900-01'])

# MATH 0900-02: Transitional Math I
# assigned to SNOW 144 at MTWR0900+50
db.make_section('MATH 0900-02', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Jameson C Hardy', ['MATH 0900-02'])

# MATH 0900-03: Transitional Math I
# assigned to SNOW 144 at MW1300+100
db.make_section('MATH 0900-03', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Paula Manuele Temple', ['MATH 0900-03'])

# MATH 0900-04: Transitional Math I
# assigned to SNOW 144 at MW1600+100
db.make_section('MATH 0900-04', ['Math lecture', 'MW1600+100'])
db.assign_faculty_sections('Scott Patrick Hicks', ['MATH 0900-04'])

# MATH 0900-06: Transitional Math I
# assigned to SNOW 3 at TR1630+100
db.make_section('MATH 0900-06', ['Math lecture', 'TR1630+100'])

# MATH 0900-07: Transitional Math I
# assigned to SNOW 144 at TR1300+100
db.make_section('MATH 0900-07', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Paula Manuele Temple', ['MATH 0900-07'])

# MATH 0900-51: Transitional Math I
# assigned to SNOW 144 at MW1800+100
db.make_section('MATH 0900-51', ['Math lecture', 'MW1800+100'])
db.assign_faculty_sections('Scott Patrick Hicks', ['MATH 0900-51'])

# MATH 0980-03: Transitional Math IIB
# assigned to SNOW 144 at MTWR1000+50
db.make_section('MATH 0980-03', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Tye K Rogers', ['MATH 0980-03'])

# MATH 0980-05: Transitional Math IIB
# assigned to SNOW 144 at TR1630+100
db.make_section('MATH 0980-05', ['Math lecture', 'TR1630+100'])
db.assign_faculty_sections('Michael N Paxman', ['MATH 0980-05'])

# MATH 0980-06: Transitional Math IIB
# assigned to SNOW 144 at MTWR0800+50
db.make_section('MATH 0980-06', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Tye K Rogers', ['MATH 0980-06'])

# MATH 0980-07: Transitional Math IIB
# assigned to SNOW 3 at MW1300+100
db.make_section('MATH 0980-07', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Kathryn E Ott', ['MATH 0980-07'])

# MATH 0980-08: Transitional Math IIB
# assigned to SNOW 3 at TR1300+100
db.make_section('MATH 0980-08', ['Math lecture', '4 credit bell schedule'])
db.assign_faculty_sections("Amanda Fa'onelua", ['MATH 0980-08'])

# MATH 0980-10: Transitional Math IIB
# assigned to SNOW 3 at MW1630+100
db.make_section('MATH 0980-10', ['Math lecture', 'MW1630+100'])

# MECH 1100-01: Manufacturing Processes
# assigned to SET 226 at MW1200+75
db.make_section('MECH 1100-01', ['SET 226', '3 credit bell schedule'])
db.assign_faculty_sections('Andrew C Schiller', ['MECH 1100-01'])

# MECH 1150-01: Prototyping Techniques
# assigned to SET 225 at TR1500+170
db.make_section('MECH 1150-01', ['SET 225', 'TR1500+170'])
db.assign_faculty_sections('Andrew C Schiller', ['MECH 1150-01'])

# MECH 1150-02: Prototyping Techniques
# assigned to SET 225 at MW1500+170
db.make_section('MECH 1150-02', ['SET 225', 'MW1500+170'])
db.assign_faculty_sections('Andrew C Schiller', ['MECH 1150-02'])

# MECH 1200-01: Coding
# assigned to SET 226 at MWF0900+50
db.make_section('MECH 1200-01', ['SET 226', '3 credit bell schedule'])
db.assign_faculty_sections('Bing Jiang', ['MECH 1200-01'])

# MECH 1200-02: Coding
# assigned to SET 226 at MWF1000+50
db.make_section('MECH 1200-02', ['SET 226', '3 credit bell schedule'])
db.assign_faculty_sections('Scott A Skeen', ['MECH 1200-02'])

# MECH 1205-01: Coding Lab
# assigned to SET 226 at R0800+110
db.make_section('MECH 1205-01', ['SET 226', '2 hour lab R0800'])
db.assign_faculty_sections('David Brent Christensen', ['MECH 1205-01'])

# MECH 1205-02: Coding Lab
# assigned to SET 226 at R1000+110
db.make_section('MECH 1205-02', ['SET 226', '2 hour lab R0800'])
db.assign_faculty_sections('David Brent Christensen', ['MECH 1205-02'])

# MECH 1205-03: Coding Lab
# assigned to SET 226 at R1200+110
db.make_section('MECH 1205-03', ['SET 226', '2 hour lab R0800'])
db.assign_faculty_sections('Russell C Reid', ['MECH 1205-03'])

# MECH 1205-04: Coding Lab
# assigned to SET 226 at R1400+110
db.make_section('MECH 1205-04', ['SET 226', '2 hour lab R0800'])
db.assign_faculty_sections('Bing Jiang', ['MECH 1205-04'])

# MECH 1205-05: Coding Lab
# assigned to SET 226 at R1600+110
db.make_section('MECH 1205-05', ['SET 226', '2 hour lab R0800'])
db.assign_faculty_sections('Bing Jiang', ['MECH 1205-05'])

# MECH 2030-01: Dynamics
# assigned to SET 104 at MWF1100+50
db.make_section('MECH 2030-01', ['SET 104', '3 credit bell schedule'])
db.assign_faculty_sections('Kameron J Eves', ['MECH 2030-01'])

# MECH 2160-01: Materials Science
# assigned to SET 226 at MW1500+75
db.make_section('MECH 2160-01', ['SET 226', '3 credit bell schedule'])
db.assign_faculty_sections('Divya Singh', ['MECH 2160-01'])

# MECH 2250-01: Sensors & Actuators
# assigned to SET 104 at MW1200+75
db.make_section('MECH 2250-01', ['SET 104', '3 credit bell schedule'])
db.assign_faculty_sections('Scott A Skeen', ['MECH 2250-01'])

# MECH 2250-02: Sensors & Actuators
# assigned to SET 104 at MW1330+75
db.make_section('MECH 2250-02', ['SET 104', '3 credit bell schedule'])
db.assign_faculty_sections('Scott A Skeen', ['MECH 2250-02'])

# MECH 2255-01: Sensors & Actuators Lab
# assigned to SET 101 at R0800+110
db.make_section('MECH 2255-01', ['SET 101', '2 hour lab R0800'])
db.assign_faculty_sections('Scott A Skeen', ['MECH 2255-01'])

# MECH 2255-02: Sensors & Actuators Lab
# assigned to SET 101 at R1200+110
db.make_section('MECH 2255-02', ['SET 101', '2 hour lab R0800'])
db.assign_faculty_sections('Scott A Skeen', ['MECH 2255-02'])

# MECH 2255-03: Sensors & Actuators Lab
# assigned to SET 101 at R1400+110
db.make_section('MECH 2255-03', ['SET 101', '2 hour lab R0800'])
db.assign_faculty_sections('David Brent Christensen', ['MECH 2255-03'])

# MECH 2255-04: Sensors & Actuators Lab
# assigned to SET 101 at R1600+110
db.make_section('MECH 2255-04', ['SET 101', '2 hour lab R0800'])
db.assign_faculty_sections('Kameron J Eves', ['MECH 2255-04'])

# MECH 3250-01: Machinery
# assigned to SET 104 at MW1630+75
db.make_section('MECH 3250-01', ['SET 104', '3 credit bell schedule'])
db.assign_faculty_sections('Divya Singh', ['MECH 3250-01'])

# MECH 3255-01: Machinery Lab
# assigned to SET 104 at T1200+110
db.make_section('MECH 3255-01', ['SET 104', '2 hour lab T0800'])
db.assign_faculty_sections('Divya Singh', ['MECH 3255-01'])

# MECH 3255-02: Machinery Lab
# assigned to SET 226 at T1200+110
db.make_section('MECH 3255-02', ['SET 226', '2 hour lab T0800'])
db.assign_faculty_sections('Andrew C Schiller', ['MECH 3255-02'])

# MECH 3600-01: Thermodynamics
# xlist entry: SC0A
# assigned to SET 104 at MTWF0900+50
db.make_section('MECH 3600-01', ['SET 104', '4 credit bell schedule'])
db.assign_faculty_sections('Russell C Reid', ['MECH 3600-01'])

# MECH 3602-01: Thermo II
# xlist entry: SC0A
# assigned to SET 104 at MTWF0900+50
db.make_section('MECH 3602-01', []) # 'SET 104', '4 credit bell schedule'
# ./edit assign-faculty-sections 'Russell C Reid', 'MECH 3602-01'
# cross-listed with MECH 3600-01

# MECH 3605-01: Thermodynamics Lab
# assigned to SET 104 at R1400+110
db.make_section('MECH 3605-01', ['SET 104', '2 hour lab R0800'])
db.assign_faculty_sections('Russell C Reid', ['MECH 3605-01'])

# MECH 3605-02: Thermodynamics Lab
# assigned to SET 104 at R1600+110
db.make_section('MECH 3605-02', ['SET 104', '2 hour lab R0800'])
db.assign_faculty_sections('Russell C Reid', ['MECH 3605-02'])

# MECH 3650-01: Heat Transfer
# assigned to SET 104 at MW1500+75
db.make_section('MECH 3650-01', ['SET 104', '3 credit bell schedule'])
db.assign_faculty_sections('Russell C Reid', ['MECH 3650-01'])

# MECH 3655-01: Heat Transfer Lab
# assigned to SET 104 at R0800+110
db.make_section('MECH 3655-01', ['SET 104', '2 hour lab R0800'])
db.assign_faculty_sections('Russell C Reid', ['MECH 3655-01'])

# MECH 3655-02: Heat Transfer Lab
# assigned to SET 104 at R1000+110
db.make_section('MECH 3655-02', ['SET 104', '2 hour lab R0800'])
db.assign_faculty_sections('Russell C Reid', ['MECH 3655-02'])

# MECH 4010-01: Product Design II
# assigned to SET 219 at MWF1330+180
db.make_section('MECH 4010-01', []) # 'SET 219', 'MWF1330+180'
# ./edit assign-faculty-sections 'Brant A Ross', 'MECH 4010-01'
# cross-listed with ECE 4010-01

# MECH 4500-01: Advanced Engineering Math
# assigned to SET 523 at TR1500+75
db.make_section('MECH 4500-01', ['SET 523', '3 credit bell schedule'])
db.assign_faculty_sections('Scott A Skeen', ['MECH 4500-01'])

# MECH 4860R-01: Design Practicum
# assigned to SET 102 at M0800+50
db.make_section('MECH 4860R-01', ['SET 102', '1 credit bell schedule'])
db.assign_faculty_sections('Scott A Skeen', ['MECH 4860R-01'])

# MECH 4990-01: Special Topics: Finite Element Analysis
# assigned to SET 523 at MW1000+110
db.make_section('MECH 4990-01', ['SET 523', '4 hour lab MW0800'])
db.assign_faculty_sections('Divya Singh', ['MECH 4990-01'])

# MTRN 2350-01: Advanced PLC Programming
# assigned to SET 102 at TR1000+50
db.make_section('MTRN 2350-01', ['SET 102', 'TR1000+50'])
db.assign_faculty_sections('Bruford P Reynolds', ['MTRN 2350-01'])

# MTRN 2355-01: Advanced PLC Programming Lab
# assigned to SET 102 at TR1400+110
db.make_section('MTRN 2355-01', ['SET 102', '4 hour lab TR0800'])
db.assign_faculty_sections('Bruford P Reynolds', ['MTRN 2355-01'])

# PHYS 1010-01: Elementary Physics (PS)
# assigned to SET 418 at MW1630+75
db.make_section('PHYS 1010-01', ['Science small lecture', '3 credit bell schedule'])
db.assign_faculty_sections('David M Syndergaard', ['PHYS 1010-01'])

# PHYS 1015-01: Elementary Physics Lab (LAB)
# assigned to SET 410 at M1300+110
db.make_section('PHYS 1015-01', ['SET 410', '2 hour lab M0900'])
db.assign_faculty_sections('David M Syndergaard', ['PHYS 1015-01'])

# PHYS 1015-02: Elementary Physics Lab (LAB)
# assigned to SET 410 at M1000+110
db.make_section('PHYS 1015-02', ['SET 410', '2 hour lab M0800'])

# PHYS 1040-50: Elementary Astronomy (PS)
# assigned to SET 418 at MW1800+75
db.make_section('PHYS 1040-50', ['Science small lecture', 'MW1800+75'])
db.assign_faculty_sections('David M Syndergaard', ['PHYS 1040-50'])

# PHYS 1045-50: Elementary Astronomy Lab (LAB)
# assigned to SET 418 at M1930+170
db.make_section('PHYS 1045-50', ['Science small lecture', 'M1930+170'])
db.assign_faculty_sections('Christopher Kirk DeMacedo', ['PHYS 1045-50'])

# PHYS 1045-51: Elementary Astronomy Lab (LAB)
# assigned to SET 418 at T1930+170
db.make_section('PHYS 1045-51', ['Science small lecture', 'T1930+170'])
db.assign_faculty_sections('Rick L Peirce', ['PHYS 1045-51'])

# PHYS 1045-52: Elementary Astronomy Lab (LAB)
# assigned to SET 418 at W1930+170
db.make_section('PHYS 1045-52', ['Science small lecture', 'W1930+170'])
db.assign_faculty_sections('Jose C Saraiva', ['PHYS 1045-52'])

# PHYS 2010-01: College Physics I (PS)
# assigned to SET 418 at MWRF0800+50
db.make_section('PHYS 2010-01', ['Science small lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Steven K Sullivan', ['PHYS 2010-01'])

# PHYS 2010-02: College Physics I (PS)
# assigned to SET 418 at MWRF1500+50
db.make_section('PHYS 2010-02', ['Science small lecture', '4 credit 4×50 extended bell schedule'])

# PHYS 2015-01: College Physics I Lab (LAB)
# assigned to SET 410 at T1200+110
db.make_section('PHYS 2015-01', ['SET 410', '2 hour lab T0800'])
db.assign_faculty_sections('Christopher Kirk DeMacedo', ['PHYS 2015-01'])

# PHYS 2015-02: College Physics I Lab (LAB)
# assigned to SET 410 at T1400+110
db.make_section('PHYS 2015-02', ['SET 410', '2 hour lab T0800'])
db.assign_faculty_sections('Christopher Kirk DeMacedo', ['PHYS 2015-02'])

# PHYS 2015-03: College Physics I Lab (LAB)
# assigned to SET 410 at T1000+110
db.make_section('PHYS 2015-03', ['SET 410', '2 hour lab T0800'])

# PHYS 2020-01: College Physics II
# assigned to SET 418 at MWRF1000+50
db.make_section('PHYS 2020-01', ['Science small lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Steven K Sullivan', ['PHYS 2020-01'])

# PHYS 2020-02: College Physics II
# assigned to SET 418 at MWRF1100+50
db.make_section('PHYS 2020-02', ['Science small lecture', '4 credit bell schedule'])
db.assign_faculty_sections('Steven K Sullivan', ['PHYS 2020-02'])

# PHYS 2025-01: College Physics II Lab
# assigned to SET 412 at T1400+50
db.make_section('PHYS 2025-01', ['SET 412', '1 credit extended bell schedule'])

# PHYS 2025-03: College Physics II Lab
# assigned to SET 412 at T1600+110
db.make_section('PHYS 2025-03', ['SET 412', '2 hour lab T0800'])
db.assign_faculty_sections('Jose C Saraiva', ['PHYS 2025-03'])

# PHYS 2025-04: College Physics II Lab
# assigned to SET 412 at T1800+110
db.make_section('PHYS 2025-04', ['SET 412', 'T1800+110'])

# PHYS 2210-01: Physics/Scientists Engineers I (PS)
# assigned to SET 418 at MTWF1300+50
db.make_section('PHYS 2210-01', ['Science small lecture', '4 credit 4×50 extended bell schedule'])
db.assign_faculty_sections('Samuel K Tobler', ['PHYS 2210-01'])

# PHYS 2210-02: Physics/Scientists Engineers I (PS)
# assigned to SET 418 at MTWF0900+50
db.make_section('PHYS 2210-02', ['Science small lecture', '4 credit bell schedule'])

# PHYS 2215-01: Physics/Scientists Engineers I Lab (LAB)
# assigned to SET 410 at R1400+110
db.make_section('PHYS 2215-01', ['SET 410', '2 hour lab R0800'])

# PHYS 2215-02: Physics/Scientists Engineers I Lab (LAB)
# assigned to SET 410 at R1600+110
db.make_section('PHYS 2215-02', ['SET 410', '2 hour lab R0800'])

# PHYS 2215-50: Physics/Scientists Engineers I Lab (LAB)
# assigned to SET 410 at R1800+110
db.make_section('PHYS 2215-50', ['SET 410', 'R1800+110'])
db.assign_faculty_sections('Jose C Saraiva', ['PHYS 2215-50'])

# PHYS 2220-01: Physics/Scientists Engineers II
# assigned to SET 418 at MTWF1400+50
db.make_section('PHYS 2220-01', ['Science small lecture', '4 credit 4×50 extended bell schedule'])
db.assign_faculty_sections('Samuel K Tobler', ['PHYS 2220-01'])

# PHYS 2225-01: Physics/Scientists Engineers II Lab
# assigned to SET 412 at R1400+110
db.make_section('PHYS 2225-01', ['SET 412', '2 hour lab R0800'])

# PHYS 2225-02: Physics/Scientists Engineers II Lab
# assigned to SET 412 at R1600+110
db.make_section('PHYS 2225-02', ['SET 412', '2 hour lab R0800'])
db.assign_faculty_sections('Jose C Saraiva', ['PHYS 2225-02'])

# PHYS 3600-01: Thermodynamics
# assigned to SET 104 at MTWF0900+50
db.make_section('PHYS 3600-01', []) # 'SET 104', '4 credit bell schedule'
# cross-listed with MECH 3600-01

# PHYS 3605-01: Thermodynamics Lab
# assigned to SET 104 at R1400+110
db.make_section('PHYS 3605-01', []) # 'SET 104', '2 hour lab R0800'
# cross-listed with MECH 3605-01

# PHYS 3605-02: Thermodynamics Lab
# assigned to SET 104 at R1600+110
db.make_section('PHYS 3605-02', []) # 'SET 104', '2 hour lab R0800'
# cross-listed with MECH 3605-02

# SCI 4700-01: Secondary Sci Teaching Methods
# assigned to SET 216 at R1530+150
db.make_section('SCI 4700-01', ['SET 216', 'R1530+150'])
db.assign_faculty_sections('Mark L Dickson', ['SCI 4700-01'])

# SCI 4720-01: Innovative Solutions - Product Development
# assigned to SET 501 at F1400+170
db.make_section('SCI 4720-01', ['SET 501', 'F1400+170'])

db.add_cross_listing('ENVS 2700R-01', ['GEO 2700R-01'])
db.add_cross_listing('ENVS 3110-01', ['GEO 3110-01'])
db.add_cross_listing('ECE 4010-01', ['MECH 4010-01'])
db.add_cross_listing('MECH 3600-01', ['MECH 3602-01', 'PHYS 3600-01'])
db.add_cross_listing('MECH 3605-01', ['PHYS 3605-01'])
db.add_cross_listing('MECH 3605-02', ['PHYS 3605-02'])
db.add_cross_listing('BIOL 1010-05', ['BIOL 1010-11'])
db.add_cross_listing('BIOL 2320-04', ['BIOL 2320-08'])
db.add_cross_listing('BIOL 3010-01', ['BIOL 3010-02'])
db.add_cross_listing('BIOL 3030-01', ['BIOL 3030-02'])
db.add_cross_listing('BTEC 2050-01', ['BTEC 2050-02'])

db.add_multiple_section_override('MATH 1050', 6)

db.db.commit()
