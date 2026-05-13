# Rust Calendar

Rust Calendar is a simple, fast and highly portable application for managing your time. It runs natively and on the web.

A distinguishing feature of Rust Calendar is an extremely flexible sharing feature. It gives the ability to give only the necessary access to other users, which can include just seeing some events that you have planned, your schedule overall, access to change it, or even full access to your account.

## Overview

A working application looks like this. 
![Working application](/images/working_app.png)
Using the burger menu on the left, users can move between their calendar and other users' calendars, manage access to their calendar, and log out.<br>
Middle part of the application can show events scheduled for the selected time period (month, week, or day), created schedules and event templates.

### Creating and editing events

Access level parameter determines the needed access level to have full information about this event (editing requires both edit permission and the access level), visibility determines what users with lower access level see (time and name, only time, or nothing).
Regular events can be edited or deleted.

### Creating recurring events

Recurring events are created with an Event Template and Schedule.

Event Template describes what an event should look like. Schedule determines time and day of the week when they should be created.<br>
Events created this way cannot be changed or deleted. Changing Schedule or Template will change all events created by them immediately.

### Granting and managing access to other users

User can grant, change or revoke access to other users. Permissions that can be granted are described [here](#access-feature).<br>
Access levels have a linear hierarchy, where any other level is either above or below another.<br>

## Access feature

Access feature is a distinguishing feature of this application; it allows user to grant only the necessary permissions to other users.

Access control is achieved using 2 features:
* Access level - allows separating events, schedules and templates in layers. Users without the necessary access level will have only chosen by the owner information.
* Permissions - allows selecting what access user will have. This includes viewing or editing events, templates or schedules, and the right to create access levels and allow sharing with other users.

## Goals

* Security - all communications between the client and the server should be secure. No important data should be stored locally in an unencrypted way. The access feature should maintain its functions without compromising security; it should never give more access than the user intended to.
* User experience - the application should be easy to work with even for inexperienced users.
* Performance and reliability - clients' applications should be responsive at all times; the server should be as performant as possible without compromising on reliability.
* Accessibility - The application should have multiple language support, the ability to use the application only with a keyboard and give all information needed for browser accessibility features to work as well.

## State

Application is in a usable state with all main features implemented. With some experience, it is a very useful time-management application.<br>
Application can be ran both natively and in web, it's fast and responsive even on low-end systems.<br>
Backend is written in an optimized and scalable way; with enough server power it will support a large active user base.<br>
Access feature is in a good state and allows for extremely flexible access sharing.

### Work in progress

UI and UX need a lot of work.<br>
Email verification is not implemented yet.<br>
Application lacks accessibility, there's no multiple language support, no way to control the application with a keyboard, and it gives no information about the UI for the browser.<br>
Notifications are not supported.<br>

<!-- ## Technical requirements WIP -->

## Technical goals

Main technical goal for this project is to achieve [Goals](##Goals) using the best programming practices I can.

The secondary goal is to create an idiomatic, fast, readable and reliable way to describe possible communications between the client and the server. This would mean creating a request definition in the [library](/lib/), adding some minimal required technical details in the [frontend](/frontend/) and [backend](/backend/) and this automatically giving strongly typed functions in the frontend and secure request handlers in the backend. This is especially useful for basic table CRUD operations, as adding them for every table is tedious and error-prone, adding them should require adding a few lines of code.

## Technical state

As of now, code is in a mostly good shape. It is overcomplicated in some places, but it does not cause problems for now.<br>
In general, application works fast, is reliable and secure.<br>
Secondary goal is still a long way from completion and requires at least one overhaul of low-level modules, but it should not cause a full application rewrite.

## Used technologies

### Frontend

* [Trunk](https://trunk-rs.github.io/trunk/) - WASM web application bundler.
* [egui](https://www.egui.rs/) - immediate mode GUI library.
* [reqwest](https://docs.rs/reqwest/latest/reqwest/) - HTTP client.
* [tokio](https://tokio.rs/) - asynchronous runtime library.

### Backend

* [Diesel](https://diesel.rs/) - ORM and query builder.
* [actix](https://actix.rs/) - URL routing, receiving HTTPS requests and sending responses.