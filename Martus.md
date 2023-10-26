# Martus 
_Yet another Open source school management software_

- [Introduction](#introduction)
- [Motivation](#motivation)
- [Requirement](#requirememts)
    - [Functional Requirements](#functional-requirements)
    - [Non Functional Requirement](#non-functional-requirements)
- [Architecture and Design](#architecture-and-design)
- [Limitations](./limitations.md)


## Introduction 
School management software, also known as a school management system (SMS) or education management software, is a comprehensive software solution designed to streamline and automate various administrative and academic tasks within an educational institution, such as schools, colleges, universities, and even online learning.

## Motivation
Building a School management backend is driven by the following needs:
- To gain a deeper understanding of software design and implementation, especially:
    - Database Design 
    - Systems architecture 
    - System Scalability and resilience 
    - Software deployment and maintenance 
- To Understand entity relation design of large-scale applications and implementation therein 
- To use Typescript, Rust, and various frameworks to build microservices 
- To gain an in-depth implementation of the following protocols:
    - HTTP
    - AMQP 
    - gRPC 
- Finally to understand the engineering decision behind the choice of service architecture, for the purpose of this learning, two services shall be considered:
    - RESTful services 
    - GraphQL 
    - Multiplex - REST + GraphQL
## Requirement
### Functional Requirements: 
Key features and functionalities of a school management software typically include:

- **Student Information Management**: It allows schools to maintain a database of student records, including personal details, enrollment information, attendance records, and academic performance.

- **Teacher and Staff Management**: This module helps institutions manage their teaching and administrative staff, including their qualifications, schedules, and payroll.

 - **Course and Curriculum Management**: Schools can create, update, and manage courses, syllabi, class schedules, and academic calendars.

 - **Attendance Tracking**: It enables tracking student and staff attendance, helping schools monitor attendance patterns and address issues promptly.

- **Grading and Assessment**: Schools can record and calculate student grades, manage assignments, quizzes, and examinations, and generate report cards.

 - **Admissions and Enrollment**: This feature streamlines the student admission process, including applications, fees, and enrollment management.

- **Financial Management**: It handles financial transactions, including fee collection, budgeting, payroll processing, and expense tracking.

- **Library Management**: Institutions can catalog books, manage library resources, track loans, and streamline library operations.

- **Timetable and Scheduling**: Helps create and manage class schedules, exams, and other events efficiently.

- **Transportation Management**: For institutions with transportation services, this module assists in managing routes, schedules, and vehicle maintenance.

- **Hostel or Dormitory Management**: If the institution offers boarding facilities, this module helps manage room assignments, occupancy, and related logistics.

 - **Inventory and Asset Management**: Tracks school assets such as computers, furniture, and other equipment.

 - **Reporting and Analytics**: Generate various reports and analytics to help administrators make data-driven decisions and assess overall performance.

- **Security and User Access Control**: Ensures data security and controls user access to sensitive information.

- **Integration and API Support**: Allows integration with other software systems, such as accounting software, learning management systems (LMS), and more.
  
- **Help Desk and Support**: Provides technical support and assistance to software users - the staff and student (possibility of a chatbot)

### Non-Functional Requirement 

- **Mobile App**:  Many school management software solutions offer mobile apps for easy access to information and communication for parents, teachers, and students.


- **Data Backup and Recovery**: Ensures that data is regularly backed up and can be recovered in case of data loss.

- **User-Friendly Interface**: A user-friendly interface is essential to make the software accessible to users with varying levels of technical expertise.

- **Communication and Messaging**: Allow for communication between teachers, parents, and students through announcements, notifications, and messaging systems.


## Architecture and Design 
See [architecture.md](./architecture.md)


## Deliverables 
The application shall be distributed as:
- Open source software 
- Open API consumable by web and mobile clients 


## Implementation and Development
The Services should be implemented using:


- Rust 
- Typescript 
- SQL databases (Postgres)


## Timeline 

1. September - October 2023:
      - Research and define software requirement 
      - Identify services to be built 
      - Split the application into sections
2. November - December
      -  Development of the application entry point and updating documentation to reflect engineering decisions made 

## Limitations
See [limitations.md](./limitations.md)

## Conclusion 
Overall, school management software aims to simplify administrative tasks, improve communication, enhance academic management, and provide valuable insights to educational institutions, allowing them to focus more on delivering quality education. The specific features and capabilities of school management software may vary from one solution to another, depending on the vendor and the needs of the institution.

