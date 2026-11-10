Department questions
====================

These questions record working assumptions in the Spring 2027 dry run. Inputs are provisional until departments review them.


Across departments
------------------

*   Which rooms are interchangeable for each course, including equipment and capacity? Science lecture rooms now allow college-owned halls at least as large as the posted room; specialized labs retain their posted room. Several capacities are provisional and use posted section caps as a lower bound.
*   The college has primary scheduling rights in SET, Snow, and Smith. Brown, COE, GCB, and UHB are treated as floated space: each remains an option for its posted section, but is not offered to other sections without a matching posted assignment. Is that consistent with current practice?
*   Which faculty can teach outside MTWR 09:00–16:30 and F 09:00–12:00? Posted meetings outside those hours are retained as section-specific times, not general availability.
*   Which two-, four-, and five-credit lecture patterns are realistic alternatives? The working time groups are inferred from the posted schedule and three-credit bell starts; labs retain their own patterns.
*   For ordinary two- and three-hour labs, the working model allows staggered daytime starts on the posted weekday or day pair. Is that appropriate for the lab sections and their instructors?
*   How should block A and B offerings interact with full-semester courses? The current solver treats weekly overlaps as conflicts even when the posted date ranges do not overlap.


Biological Sciences
-------------------

*   BIOL 4380 is listed as two credits in the catalog and three credits in the posted schedule. The working catalog permits both values; which is correct?
*   BIOL 2420-03 and -07 share their lecture and have distinct unmarked SI meetings. The dry run models one shared lecture and separate SI meetings; confirm the arrangement and available rooms.
*   BIOL 1010-02/10, 1200-01/04, and 2320-02/06, 03/08, and 04/09 have matching lecture times and rooms with different SI meetings. The model treats each pair as one lecture; confirm these groupings.
*   BTEC 2050-01 and -02 share an 11:00 Tuesday meeting that ends at 12:50 for section 01 and 11:50 for section 02. The working model reserves the common meeting through 12:50.
*   Which completed courses are reasonable de-facto prerequisites for BIOL 4910 Senior Seminar across biology degrees? It is temporarily omitted from undergraduate conflict groups to avoid conflicts with introductory classes.
*   Confirm whether biology graduate 5000/6000 pairs meeting at the same time are shared sections and whether the graduate emphasis courses are all required.
*   BIOL 1615A names BIOL 1610A as a corequisite, but BIOL 1610A has no current catalog description. Is there a current equivalent?
*   The 2024 model included Biology with Integrated Science Education. The current Education listing uses that name but links to an Integrated Studies biology page; it is omitted under the Integrated Studies exclusion. Is there a separate current Biology Education degree that should be modeled?


Chemistry and Biochemistry
--------------------------

*   CHEM 3300-01 has an unmarked Thursday meeting. The working assumption treats it as supplemental instruction led without the instructor; confirm that interpretation.
*   CHEM 1210-02 and -04 share a posted lecture in SET 201, while their SI sessions differ and section 04 lists its instructor as TBA. The model treats them as one lecture with two SI meetings; confirm that arrangement.
*   CHEM 4510-01 has separate lecture and lab meetings under one CRN. The working model fixes both meetings and assigns the instructor to each.
*   CHEM 4800R independent-research sections have posted weekly times that overlap other classes taught by their instructors. They are recorded as unscheduled workload; are the posted meetings actual fixed class meetings?


Engineering and Physics
-----------------------

*   Are the inferred MTWF and MWRF four-credit physics lecture times interchangeable? The working model allows daytime starts and retains the posted time.
*   ECE 4010-01, ECE 4015-01, MTRN 4010-01, and MECH 4010-01 share a posted time and room. The model treats them as one product-design meeting with both posted instructors.
*   MECH 3600-01 and MECH 3602-01 share a posted time and room, but carry four and one credits respectively. The model treats them as one meeting with the primary section's four-credit value for solver output; each section retains its own recorded credits.
*   MTRN 3455-01 has two Tuesday lab meetings separated by ten minutes. The model retains both fixed meetings and the same instructor.
*   MECH 4860R-02 is posted during the instructor's shared product-design meeting in the same room. It is treated as unscheduled practicum workload; does it meet with that class or need a separate placement?
*   Current descriptions still refer to absent MECH 2210, MECH 2215, MECH 3415, MECH 4500, ECE 2100, ECE 2280, and MTRN 3360. Most appear as alternatives to current courses; ECE 2215 and MTRN 3415 are used as lab corrections. Which references should be replaced or removed?


Mathematics
-----------

*   The Mathematics BA/BS worksheet lists MATH 4250, but the current catalog has no course description for it. Should it remain an elective?
*   Confirm whether the Snow mathematics rooms can all be used interchangeably and whether the inferred four- and five-credit patterns fit department practice.
*   Four-credit mathematics meetings are posted through 16:40. The working model extends all mathematics faculty's MTWR availability to 16:40; is that reasonable?
*   The 2024 model included an Applied and Computational Mathematics scientific-computing emphasis. The current college program list has only the actuarial and data-analytics emphases, although its description still mentions scientific computing. Is it still an active degree option?


Design
------

*   Which design rooms and equipment are interchangeable? Smith 114, Smith 127, and AIP 139 are fixed to their posted sections, with provisional capacities based on section caps.
*   VIRT sections with posted days and times are treated as synchronous meetings without a physical room. DES 1610-50 has a time but no posted room and is also scheduled without one. Are those assumptions correct?
*   DES 4600-01 and -02 share a room and time with different instructors. The model treats them as a shared meeting taught by both; confirm that arrangement.
*   DES 2300-01 and DES 3600-01 have posted weekly minutes exceeding their three-credit lecture pattern. Their posted patterns are retained; confirm their intended meeting arrangements.
*   The DES 4600 senior-project prerequisite overrides use earlier common design requirements to represent senior standing. Do those choices match actual student progression?
*   DES 2780-01 is posted in AIP 139, probably the Atwood Innovation Plaza makerspace. Its room remains fixed but its time may use the three-credit bell schedule; confirm this flexibility and space requirement.


Earth, Energy and Environmental Science
--------------------------------------

*   The Geoscience worksheet lists ENVS 4085, but the current catalog has no course description for it. Is it still an elective?
*   ENER 4310 names ENER 2310 as a prerequisite, but ENER 2310 has no current catalog description. Is there a current equivalent?
*   Which courses should serve as de-facto prerequisites for ENVS 4910 Senior Seminar? It is temporarily omitted from conflict groups.
*   ENVS 4080-01 and GEO 3700-01 each have two posted meetings. The model fixes both meetings with their posted instructor; confirm whether they should move together or have other permissible times.
*   ENVS 4910-01 and GEO 4910-01 share a posted room, time, and instructor. The model treats them as one meeting; confirm they are shared sections.
*   GEO 1010-04H meets MW 09:45–11:00 at Hurricane Center and lists its instructor as TBA. This is the only HURCTR section in the supplied files, so no faculty-specific morning or travel pattern can be inferred. Its time and room remain fixed, and the solver does not account for inter-campus travel.
