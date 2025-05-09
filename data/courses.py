import queries

def build_courses(db: queries.DB) -> None:
    print('building departments')
    db.make_department('Computing')
    db.make_department('Math')

    print('building courses')
    db.make_course('Computing', 'CS 1030', 'Problem Solving with Computers')
    db.make_course('Computing', 'CS 1400', 'Fundamentals of Programming')
    db.add_course_rotation('CS 1400', 'fall')
    db.add_course_rotation('CS 1400', 'spring')
    db.add_course_rotation('CS 1400', 'summer')
    db.make_course('Computing', 'CS 1410', 'Object Oriented Programming')
    db.add_course_rotation('CS 1410', 'fall')
    db.add_course_rotation('CS 1410', 'spring')
    db.make_course('Computing', 'CS 2100', 'Discrete Structures')
    db.add_course_rotation('CS 2100', 'fall')
    db.make_course('Computing', 'CS 2320', 'Introduction to Machine Learning')
    db.add_course_rotation('CS 2320', 'spring')
    db.make_course('Computing', 'CS 2420', 'Introduction to Algorithms and Data Structures')
    db.add_course_rotation('CS 2420', 'fall')
    db.add_course_rotation('CS 2420', 'spring')
    db.make_course('Computing', 'CS 2450', 'Software Engineering')
    db.add_course_rotation('CS 2450', 'fall')
    db.add_course_rotation('CS 2450', 'spring')
    db.make_course('Computing', 'CS 2500', 'Data Wrangling')
    db.add_course_rotation('CS 2500', 'fall')
    db.make_course('Computing', 'CS 2810', 'Computer Organization and Architecture')
    db.add_course_rotation('CS 2810', 'fall')
    db.add_course_rotation('CS 2810', 'spring')
    db.make_course('Computing', 'CS 3005', 'Programming in C++')
    db.add_course_rotation('CS 3005', 'fall')
    db.add_course_rotation('CS 3005', 'spring')
    db.make_course('Computing', 'CS 3150', 'Computer Networks')
    db.add_course_rotation('CS 3150', 'spring')
    db.make_course('Computing', 'CS 3400', 'Operating Systems')
    db.add_course_rotation('CS 3400', 'fall')
    db.make_course('Computing', 'CS 3410', 'Distributed Systems')
    db.add_course_rotation('CS 3410', 'spring')
    db.make_course('Computing', 'CS 3500', 'Game Development')
    db.add_course_rotation('CS 3500', 'fall')
    db.make_course('Computing', 'CS 3510', 'Algorithms')
    db.add_course_rotation('CS 3510', 'spring')
    db.make_course('Computing', 'CS 3520', 'Programming Languages')
    db.add_course_rotation('CS 3520', 'fall')
    db.make_course('Computing', 'CS 3530', 'Computational Theory')
    db.add_course_rotation('CS 3530', 'fall')
    db.make_course('Computing', 'CS 3600', 'Graphics Programming')
    db.add_course_rotation('CS 3600', 'spring')
    db.make_course('Computing', 'CS 4300', 'Artificial Intelligence')
    db.add_course_rotation('CS 4300', 'fall')
    db.make_course('Computing', 'CS 4307', 'Database Systems')
    db.add_course_rotation('CS 4307', 'spring')
    db.make_course('Computing', 'CS 4320', 'Machine Learning')
    db.add_course_rotation('CS 4320', 'spring')
    db.make_course('Computing', 'CS 4400', 'Data Mining')
    db.add_course_rotation('CS 4400', 'fall')
    db.make_course('Computing', 'CS 4410', 'Data Visualization')
    db.add_course_rotation('CS 4410', 'fall')
    db.make_course('Computing', 'CS 4550', 'Compilers')
    db.add_course_rotation('CS 4550', 'spring')
    db.make_course('Computing', 'CS 4600', 'Senior Project')
    db.make_course('Computing', 'CS 4800R', 'Undergraduate Research')
    db.add_course_rotation('CS 4800R', 'fall')
    db.add_course_rotation('CS 4800R', 'spring')
    db.make_course('Computing', 'CS 4920R', 'Internship')
    db.make_course('Computing', 'CS 4990', 'Special Topics in Computer Science')
    db.make_course('Computing', 'CS 4991R', 'Competitive Programming')
    db.add_course_rotation('CS 4991R', 'fall')
    db.add_course_rotation('CS 4991R', 'spring')
    db.make_course('Computing', 'CS 4992R', 'Computer Science Seminar')
    db.add_course_rotation('CS 4992R', 'fall')
    db.add_course_rotation('CS 4992R', 'spring')
    db.make_course('Computing', 'CS 4995', 'Programming for VR/XR Internship 01')
    db.add_course_rotation('CS 4995', 'fall')
    db.make_course('Computing', 'CS 4996', 'Programming for VR/XR Internship 02')
    db.add_course_rotation('CS 4996', 'spring')
    db.make_course('Computing', 'IT 1100', 'Introduction to Unix/Linux')
    db.add_course_rotation('IT 1100', 'fall')
    db.add_course_rotation('IT 1100', 'spring')
    db.make_course('Computing', 'IT 1200', 'A+ Computer Hardware/Windows OS')
    db.add_course_rotation('IT 1200', 'fall')
    db.add_course_rotation('IT 1200', 'spring')
    db.make_course('Computing', 'IT 1500', 'Cloud Fundamentals')
    db.add_course_rotation('IT 1500', 'fall')
    db.add_course_rotation('IT 1500', 'spring')
    db.make_course('Computing', 'IT 2300', 'Database Design & Management')
    db.add_course_rotation('IT 2300', 'fall')
    db.add_course_rotation('IT 2300', 'spring')
    db.make_course('Computing', 'IT 2400', 'Intro to Networking')
    db.add_course_rotation('IT 2400', 'fall')
    db.add_course_rotation('IT 2400', 'spring')
    db.make_course('Computing', 'IT 2500', 'Cloud Computing')
    db.add_course_rotation('IT 2500', 'fall')
    db.make_course('Computing', 'IT 2700', 'Information Security')
    db.add_course_rotation('IT 2700', 'fall')
    db.add_course_rotation('IT 2700', 'spring')
    db.make_course('Computing', 'IT 3001', 'Info Sys and Analytics Intermediate Career Strategies')
    db.add_course_rotation('IT 3001', 'fall')
    db.add_course_rotation('IT 3001', 'spring')
    db.make_course('Computing', 'IT 3100', 'Systems Design and Administration')
    db.add_course_rotation('IT 3100', 'fall')
    db.make_course('Computing', 'IT 3110', 'System Automation')
    db.add_course_rotation('IT 3110', 'spring')
    db.make_course('Computing', 'IT 3150', 'Windows Servers')
    db.add_course_rotation('IT 3150', 'spring')
    db.make_course('Computing', 'IT 3300', 'DevOps Virtualization')
    db.add_course_rotation('IT 3300', 'fall')
    db.make_course('Computing', 'IT 3400', 'Intermediate Computer Networking')
    db.add_course_rotation('IT 3400', 'spring')
    db.make_course('Computing', 'IT 3710', 'Network Defense')
    db.add_course_rotation('IT 3710', 'spring')
    db.make_course('Computing', 'IT 4060', 'Big Data Analytics')
    db.add_course_rotation('IT 4060', 'fall')
    db.make_course('Computing', 'IT 4070', 'Data Visualization and Storytelling')
    db.add_course_rotation('IT 4070', 'spring')
    db.make_course('Computing', 'IT 4100', 'Files Systems and Storage Technologies')
    db.make_course('Computing', 'IT 4200', 'DevOps Lifecycle Management')
    db.add_course_rotation('IT 4200', 'fall')
    db.make_course('Computing', 'IT 4310', 'Database Administration')
    db.add_course_rotation('IT 4310', 'fall')
    db.make_course('Computing', 'IT 4400', 'Network Design & Management')
    db.add_course_rotation('IT 4400', 'fall')
    db.make_course('Computing', 'IT 4510', 'Ethical Hacking & Network Defense')
    db.add_course_rotation('IT 4510', 'spring')
    db.make_course('Computing', 'IT 4600', 'Senior Capstone')
    db.add_course_rotation('IT 4600', 'spring')
    db.make_course('Computing', 'IT 4910R', 'Special Topics in Applied Technology')
    db.make_course('Computing', 'IT 4920R', 'Internship')
    db.add_course_rotation('IT 4920R', 'fall')
    db.add_course_rotation('IT 4920R', 'spring')
    db.add_course_rotation('IT 4920R', 'summer')
    db.make_course('Computing', 'IT 4990', 'Special Topics in Information Technology')
    db.make_course('Computing', 'IT 4991', 'Seminar in Information Technology')
    db.make_course('Math', 'MATH 0900', 'Transitional Math I')
    db.add_course_rotation('MATH 0900', 'fall')
    db.add_course_rotation('MATH 0900', 'spring')
    db.add_course_rotation('MATH 0900', 'summer')
    db.make_course('Math', 'MATH 0980', 'Transitional Math IIB')
    db.add_course_rotation('MATH 0980', 'fall')
    db.add_course_rotation('MATH 0980', 'spring')
    db.add_course_rotation('MATH 0980', 'summer')
    db.make_course('Math', 'MATH 1010', 'Intermediate Algebra')
    db.add_course_rotation('MATH 1010', 'fall')
    db.add_course_rotation('MATH 1010', 'spring')
    db.add_course_rotation('MATH 1010', 'summer')
    db.make_course('Math', 'MATH 1020R', 'Bridge Into College Mathematics')
    db.add_course_rotation('MATH 1020R', 'summer')
    db.make_course('Math', 'MATH 1030', 'Quantitative Reasoning (MA)')
    db.add_course_rotation('MATH 1030', 'fall')
    db.add_course_rotation('MATH 1030', 'spring')
    db.add_course_rotation('MATH 1030', 'summer')
    db.make_course('Math', 'MATH 1040', 'Introduction to Statistics (MA)')
    db.add_course_rotation('MATH 1040', 'fall')
    db.add_course_rotation('MATH 1040', 'spring')
    db.add_course_rotation('MATH 1040', 'summer')
    db.make_course('Math', 'MATH 1050', 'College Algebra / Pre-Calculus (MA)')
    db.add_course_rotation('MATH 1050', 'fall')
    db.add_course_rotation('MATH 1050', 'spring')
    db.add_course_rotation('MATH 1050', 'summer')
    db.make_course('Math', 'MATH 1060', 'Trigonometry (MA)')
    db.add_course_rotation('MATH 1060', 'fall')
    db.add_course_rotation('MATH 1060', 'spring')
    db.add_course_rotation('MATH 1060', 'summer')
    db.make_course('Math', 'MATH 1080', 'Pre-Calculus with Trigonometry (MA)')
    db.add_course_rotation('MATH 1080', 'fall')
    db.add_course_rotation('MATH 1080', 'spring')
    db.make_course('Math', 'MATH 1100', 'Business Calculus (MA)')
    db.add_course_rotation('MATH 1100', 'fall')
    db.add_course_rotation('MATH 1100', 'spring')
    db.add_course_rotation('MATH 1100', 'summer')
    db.make_course('Math', 'MATH 1210', 'Calculus I (MA)')
    db.add_course_rotation('MATH 1210', 'fall')
    db.add_course_rotation('MATH 1210', 'spring')
    db.make_course('Math', 'MATH 1220', 'Calculus II (MA)')
    db.add_course_rotation('MATH 1220', 'fall')
    db.add_course_rotation('MATH 1220', 'spring')
    db.make_course('Math', 'MATH 2010', 'Math for Elementary Teachers I')
    db.add_course_rotation('MATH 2010', 'fall')
    db.add_course_rotation('MATH 2010', 'spring')
    db.make_course('Math', 'MATH 2020', 'Math for Elemen Teachers II')
    db.make_course('Math', 'MATH 2050', 'Applied Statistics with Programming')
    db.add_course_rotation('MATH 2050', 'fall')
    db.make_course('Math', 'MATH 2200', 'Discrete Mathematics')
    db.add_course_rotation('MATH 2200', 'spring')
    db.make_course('Math', 'MATH 2210', 'Multivariable Calculus (MA)')
    db.add_course_rotation('MATH 2210', 'fall')
    db.add_course_rotation('MATH 2210', 'spring')
    db.make_course('Math', 'MATH 2250', 'Differential Equations and Linear Algebra')
    db.add_course_rotation('MATH 2250', 'spring')
    db.make_course('Math', 'MATH 2270', 'Linear Algebra')
    db.make_course('Math', 'MATH 2280', 'Ordinary Differential Equations')
    db.make_course('Math', 'MATH 2285', 'Adventures in Modeling')
    db.add_course_rotation('MATH 2285', 'fall')
    db.make_course('Math', 'MATH 2905', 'Survey of Cryptography')
    db.make_course('Math', 'MATH 3000', 'History of Mathematics')
    db.make_course('Math', 'MATH 3010', 'Algebra for Secondary Mathematics Teaching')
    db.make_course('Math', 'MATH 3020', 'Geometry and Statistics for Secondary Mathematics Teaching')
    db.make_course('Math', 'MATH 3050', 'Stochastic Modeling and Applications')
    db.add_course_rotation('MATH 3050', 'spring')
    db.make_course('Math', 'MATH 3060', 'Statistics for Scientists')
    db.add_course_rotation('MATH 3060', 'fall')
    db.add_course_rotation('MATH 3060', 'spring')
    db.make_course('Math', 'MATH 3100', 'Euclidean / Non-Euclidean Geom')
    db.make_course('Math', 'MATH 3120', 'Transition to Advanced Mathematics')
    db.add_course_rotation('MATH 3120', 'fall')
    db.make_course('Math', 'MATH 3150', 'Introduction to Partial Differential Equations')
    db.make_course('Math', 'MATH 3200', 'Introduction to Analysis I')
    db.add_course_rotation('MATH 3200', 'spring')
    db.make_course('Math', 'MATH 3210', 'Introduction to Analysis II')
    db.make_course('Math', 'MATH 3400', 'Probability & Statistics')
    db.add_course_rotation('MATH 3400', 'fall')
    db.make_course('Math', 'MATH 3410', 'Actuarial Exam P/1 Preparation')
    db.add_course_rotation('MATH 3410', 'fall')
    db.make_course('Math', 'MATH 3450', 'Statistical Inference')
    db.add_course_rotation('MATH 3450', 'spring')
    db.make_course('Math', 'MATH 3500', 'Numerical Analysis')
    db.make_course('Math', 'MATH 3605', 'Introduction to Modeling and Simulation')
    db.make_course('Math', 'MATH 3700', 'Mathematical Modeling I')
    db.make_course('Math', 'MATH 3900', 'Number Theory')
    db.make_course('Math', 'MATH 3905', 'Cryptography and Codes')
    db.make_course('Math', 'MATH 4000', 'Foundations of Algebra')
    db.add_course_rotation('MATH 4000', 'fall')
    db.make_course('Math', 'MATH 4005', 'Quantum Computing and Cryptography')
    db.make_course('Math', 'MATH 4010', 'Abstract Algebra II')
    db.make_course('Math', 'MATH 4100', 'Introduction to Topology')
    db.make_course('Math', 'MATH 4200', 'Introduction to Complex Analysis')
    db.make_course('Math', 'MATH 4250', 'Programming for Scientific Computation')
    db.add_course_rotation('MATH 4250', 'spring')
    db.make_course('Math', 'MATH 4330', 'Linear Algebra II')
    db.make_course('Math', 'MATH 4400', 'Financial Mathematics')
    db.add_course_rotation('MATH 4400', 'spring')
    db.make_course('Math', 'MATH 4410', 'Actuarial Exam FM/ 2 Preparation')
    db.add_course_rotation('MATH 4410', 'spring')
    db.make_course('Math', 'MATH 4450', 'Math for Secondary Special Education Teachers')
    db.add_course_rotation('MATH 4450', 'fall')
    db.add_course_rotation('MATH 4450', 'spring')
    db.make_course('Math', 'MATH 4500', 'Methods Teach Secondary Math')
    db.add_course_rotation('MATH 4500', 'fall')
    db.make_course('Math', 'MATH 4550', 'Scientific Computation')
    db.make_course('Math', 'MATH 4800', 'Industrial Careers in Mathematics')
    db.add_course_rotation('MATH 4800', 'spring')
    db.make_course('Math', 'MATH 4890R', 'Independent Research')
    db.add_course_rotation('MATH 4890R', 'fall')
    db.add_course_rotation('MATH 4890R', 'spring')
    db.add_course_rotation('MATH 4890R', 'summer')
    db.make_course('Math', 'MATH 4900', 'Senior Capstone Seminar (ALUR)')
    db.add_course_rotation('MATH 4900', 'fall')
    db.add_course_rotation('MATH 4900', 'spring')
    db.make_course('Computing', 'SD 6100', 'Fundamentals of Programming')
    db.make_course('Computing', 'SD 6110', 'Foundations of UI/UX Design')
    db.make_course('Computing', 'SD 6200', 'Multitier App Development I')
    db.make_course('Computing', 'SD 6210', 'Tech Entrepreneurship')
    db.make_course('Computing', 'SD 6220', 'Software Development Practices')
    db.make_course('Computing', 'SD 6300', 'Multitier App Development II')
    db.make_course('Computing', 'SD 6310', 'Software Quality and Testing')
    db.make_course('Computing', 'SD 6330', 'Mobile App Development for Android')
    db.make_course('Computing', 'SD 6340', 'Mobile App Development for iOS')
    db.make_course('Computing', 'SD 6400', 'Advanced Topics in App Development')
    db.make_course('Computing', 'SD 6450', 'Graduate Capstone')
    db.make_course('Computing', 'SE 1400', 'Web Design Fundamentals (ALCS)')
    db.add_course_rotation('SE 1400', 'fall')
    db.add_course_rotation('SE 1400', 'spring')
    db.make_course('Computing', 'SE 3010', 'Mobile Application Development for Android')
    db.add_course_rotation('SE 3010', 'spring')
    db.make_course('Computing', 'SE 3020', 'Mobile Application Development for iOS')
    db.add_course_rotation('SE 3020', 'fall')
    db.make_course('Computing', 'SE 3100', 'Software Practices')
    db.add_course_rotation('SE 3100', 'spring')
    db.make_course('Computing', 'SE 3150', 'Software Quality')
    db.add_course_rotation('SE 3150', 'fall')
    db.make_course('Computing', 'SE 3200', 'Web Application Development I')
    db.add_course_rotation('SE 3200', 'fall')
    db.add_course_rotation('SE 3200', 'spring')
    db.make_course('Computing', 'SE 3250', 'Internet of Things Programming')
    db.make_course('Computing', 'SE 3400', 'Human-Computer Interaction')
    db.add_course_rotation('SE 3400', 'fall')
    db.make_course('Computing', 'SE 3450', 'User Experience Design')
    db.add_course_rotation('SE 3450', 'spring')
    db.make_course('Computing', 'SE 3500', 'Tech Entrepreneurship')
    db.add_course_rotation('SE 3500', 'fall')
    db.add_course_rotation('SE 3500', 'spring')
    db.make_course('Computing', 'SE 3550', 'Online Marketing and SEO (ALCS)')
    db.add_course_rotation('SE 3550', 'fall')
    db.add_course_rotation('SE 3550', 'spring')
    db.make_course('Computing', 'SE 4200', 'Web Application Development II')
    db.add_course_rotation('SE 4200', 'spring')
    db.make_course('Computing', 'SE 4600', 'Senior Project')
    db.add_course_rotation('SE 4600', 'spring')
    db.make_course('Computing', 'SE 4900R', 'Independent Research')
    db.make_course('Computing', 'SE 4910R', 'Special Topics in Applied Technology')
    db.make_course('Computing', 'SE 4920', 'Internship (ALPP)')
    db.add_course_rotation('SE 4920', 'fall')
    db.add_course_rotation('SE 4920', 'spring')
    db.add_course_rotation('SE 4920', 'summer')
    db.make_course('Computing', 'SE 4990', 'Special Topics in Software Engineering')
    db.make_course('Computing', 'SET 1000', 'Graduation Planning & Career Prep I')
    db.add_course_rotation('SET 1000', 'fall')
    db.add_course_rotation('SET 1000', 'spring')

    print('adding prereqs')
    db.add_prereqs('CS 1400', ['CS 1030', 'MATH 1010'])
    db.add_prereqs('CS 1410', ['CS 1400'])
    db.add_prereqs('CS 2100', ['MATH 1100', 'MATH 1210', 'CS 1410'])
    db.add_prereqs('CS 2320', ['CS 1400'])
    db.add_prereqs('CS 2420', ['CS 1410'])
    db.add_prereqs('CS 2450', ['CS 1410'])
    db.add_prereqs('CS 2500', ['CS 1410'])
    db.add_prereqs('CS 2810', ['CS 1410'])
    db.add_prereqs('CS 3005', ['CS 1410'])
    db.add_prereqs('CS 3150', ['CS 2420', 'CS 2810'])
    db.add_prereqs('CS 3400', ['CS 2420', 'CS 2810', 'CS 3005'])
    db.add_prereqs('CS 3410', ['CS 2420', 'CS 2810'])
    db.add_prereqs('CS 3500', ['CS 3005'])
    db.add_prereqs('CS 3510', ['CS 2420', 'CS 2810'])
    db.add_prereqs('CS 3520', ['CS 2420', 'CS 2810'])
    db.add_prereqs('CS 3530', ['CS 2420', 'CS 2810', 'CS 2100'])
    db.add_prereqs('CS 3600', ['CS 2420', 'CS 3005'])
    db.add_prereqs('CS 4300', ['CS 2420', 'CS 2810', 'CS 3005'])
    db.add_prereqs('CS 4307', ['CS 2420', 'CS 2810'])
    db.add_prereqs('CS 4320', ['CS 2420', 'CS 2810', 'CS 3005'])
    db.add_prereqs('CS 4400', ['CS 2420', 'CS 2810'])
    db.add_prereqs('CS 4410', ['CS 2420', 'CS 2810'])
    db.add_prereqs('CS 4550', ['CS 2420', 'CS 2810', 'CS 3005'])
    db.add_prereqs('CS 4600', ['CS 2420', 'CS 2810', 'CS 3005'])
    db.add_prereqs('CS 4800R', ['CS 2420', 'CS 2810'])
    db.add_prereqs('CS 4920R', ['CS 2420', 'CS 2810', 'CS 3005'])
    db.add_prereqs('CS 4991R', ['CS 1400'])
    db.add_prereqs('CS 4992R', ['CS 2420', 'CS 2810'])
    db.add_prereqs('CS 4995', ['CS 3500', 'CS 3600'])
    db.add_prereqs('CS 4996', ['CS 4995'])
    db.add_prereqs('IT 2300', ['CS 1400', 'IT 1100', 'CS 1410'])
    db.add_prereqs('IT 2400', ['IT 1100', 'IT 1500'])
    db.add_prereqs('IT 2500', ['IT 2400', 'IT 1500'])
    db.add_prereqs('IT 2700', ['CS 1400', 'IT 2400'])
    db.add_prereqs('IT 3100', ['CS 1400', 'IT 2400', 'IT 1100', 'IT 1500', 'CS 3150'])
    db.add_prereqs('IT 3110', ['IT 3100', 'CS 1410'])
    db.add_prereqs('IT 3150', ['IT 2400'])
    db.add_prereqs('IT 3300', ['IT 2400', 'IT 1100', 'CS 3150'])
    db.add_prereqs('IT 3400', ['IT 2400'])
    db.add_prereqs('IT 3710', ['IT 2700'])
    db.add_prereqs('IT 4060', ['MATH 1040'])
    db.add_prereqs('IT 4100', ['IT 3100'])
    db.add_prereqs('IT 4200', ['CS 1400', 'IT 2400', 'CS 2810'])
    db.add_prereqs('IT 4310', ['IT 2300'])
    db.add_prereqs('IT 4400', ['IT 3400'])
    db.add_prereqs('IT 4510', ['CS 1410', 'IT 3100'])
    db.add_prereqs('IT 4600', ['CS 1410', 'IT 2400'])
    db.add_prereqs('MATH 1010', ['MATH 0900', 'MATH 0980'])
    db.add_prereqs('MATH 1030', ['MATH 0980', 'MATH 1010'])
    db.add_prereqs('MATH 1040', ['MATH 0980'])
    db.add_prereqs('MATH 1050', ['MATH 1010'])
    db.add_prereqs('MATH 1060', ['MATH 1050'])
    db.add_prereqs('MATH 1080', ['MATH 1010'])
    db.add_prereqs('MATH 1100', ['MATH 1050'])
    db.add_prereqs('MATH 1210', ['MATH 1050', 'MATH 1060', 'MATH 1080'])
    db.add_prereqs('MATH 1220', ['MATH 1210'])
    db.add_prereqs('MATH 2010', ['MATH 1030', 'MATH 1050'])
    db.add_prereqs('MATH 2020', ['MATH 2010'])
    db.add_prereqs('MATH 2050', ['MATH 1040'])
    db.add_prereqs('MATH 2200', ['MATH 1210'])
    db.add_prereqs('MATH 2210', ['MATH 1220'])
    db.add_prereqs('MATH 2250', ['MATH 1220'])
    db.add_prereqs('MATH 2270', ['MATH 1210'])
    db.add_prereqs('MATH 2280', ['MATH 1220'])
    db.add_prereqs('MATH 2285', ['MATH 1210'])
    db.add_prereqs('MATH 2905', ['MATH 1210', 'CS 1400'])
    db.add_prereqs('MATH 3000', ['MATH 1220'])
    db.add_prereqs('MATH 3010', ['MATH 1210'])
    db.add_prereqs('MATH 3020', ['MATH 1210'])
    db.add_prereqs('MATH 3050', ['MATH 2050', 'MATH 3060'])
    db.add_prereqs('MATH 3100', ['MATH 2200'])
    db.add_prereqs('MATH 3120', ['MATH 2200', 'CS 2100', 'MATH 1220'])
    db.add_prereqs('MATH 3150', ['MATH 2210', 'MATH 2270', 'MATH 2280'])
    db.add_prereqs('MATH 3200', ['MATH 3120', 'MATH 1220'])
    db.add_prereqs('MATH 3210', ['MATH 3200', 'MATH 2210'])
    db.add_prereqs('MATH 3400', ['MATH 1210'])
    db.add_prereqs('MATH 3410', ['MATH 3400'])
    db.add_prereqs('MATH 3450', ['MATH 3400'])
    db.add_prereqs('MATH 3500', ['MATH 2270', 'MATH 2280', 'MATH 2250'])
    db.add_prereqs('MATH 3605', ['MATH 1210'])
    db.add_prereqs('MATH 3700', ['MATH 2280'])
    db.add_prereqs('MATH 3900', ['MATH 1210', 'MATH 2200', 'CS 2100'])
    db.add_prereqs('MATH 3905', ['CS 1400', 'MATH 2200', 'CS 2100'])
    db.add_prereqs('MATH 4000', ['MATH 2270', 'MATH 3120'])
    db.add_prereqs('MATH 4005', ['CS 1400', 'MATH 2250', 'MATH 2270'])
    db.add_prereqs('MATH 4010', ['MATH 4000'])
    db.add_prereqs('MATH 4100', ['MATH 2210', 'MATH 3120'])
    db.add_prereqs('MATH 4200', ['MATH 3200'])
    db.add_prereqs('MATH 4250', ['CS 1400', 'MATH 2270'])
    db.add_prereqs('MATH 4330', ['MATH 2270', 'MATH 3120'])
    db.add_prereqs('MATH 4400', ['MATH 1100', 'MATH 1210'])
    db.add_prereqs('MATH 4410', ['MATH 4400'])
    db.add_prereqs('MATH 4500', ['MATH 1210'])
    db.add_prereqs('MATH 4550', ['MATH 3500'])
    db.add_prereqs('SE 3010', ['CS 2420', 'CS 3005'])
    db.add_prereqs('SE 3020', ['CS 2420', 'CS 3005'])
    db.add_prereqs('SE 3100', ['CS 2450'])
    db.add_prereqs('SE 3150', ['CS 2450'])
    db.add_prereqs('SE 3200', ['CS 1410', 'SE 1400', 'CS 2810'])
    db.add_prereqs('SE 3250', ['CS 1410'])
    db.add_prereqs('SE 3400', ['SE 1400'])
    db.add_prereqs('SE 3450', ['SE 1400'])
    db.add_prereqs('SE 4200', ['SE 3200'])
    db.add_prereqs('SE 4600', ['CS 2420', 'CS 2450', 'CS 2810', 'CS 3005', 'SE 1400', 'SE 3200'])

    print('adding coreqs')
    db.add_coreqs('CS 2810', ['CS 1410'])
    db.add_coreqs('IT 2400', ['IT 1100', 'IT 1500'])
    db.add_coreqs('MATH 4250', ['MATH 2280'])
