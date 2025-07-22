# 🛠️ Development Log | TaskBoard FULL-STACK

Here I share my thoughts, updates, technical decisions and notes from the development process.

---

## 📌 Overview
**Title:** TaskBoard FULL-STACK<br>
**Description:** Simple task management app - with user login, list of tasks, add, view and delete tasks etc...<br>
**Technology:** Rust, SQLite<br>
**Repository:** [\[Link to repository\]](https://github.com/JustMipe/TaskBoard-full-stack-)

---

## 🔍 Project goal
- [**Why**]Project is used to show my experience with Rust, technologies and AI.
- [**And**]The project will serve as a desktop GUI application that will communication with the server via HTTP API.
- [**Future**] – I would like to expand it with more features, tweak the details and let it grow with me.<br>I want to achieve a full-fledged safety-focused project.

---

<div align="center">

## 📅 Development history & Updates
### 🚀 Version 0.1 – [22.7.2025]
</div>

- 🆕 **What's new:** Ready Rust project for the backend, SQLite database setup, A ready-made users table, Dependencies for the web server, Database, Hashing and JWT, The API server is running, Connected to the database, Processed JSON from request, Password is hashed securely, Data is stored in DB,  Response is in JSON format, Register endpoint, We hash the passwords and store them in the database.<br>
- 📝 **Note:** It was a ride. The server has been created and is working as expected, it is now processing the registration request and creating users in the local database. Stay tuned, there will be more to come!

<div align="center">

![Demo](https://github.com/JustMipe/TaskBoard-full-stack-/blob/main/backend/assets/001.gif)
</div>

---

### 🛠️ Work in Progress – [Current state]
- Create a /login endpoint - which authenticates the user and issues a JWT token.
- Organizing code (split into routes, models, auth, db).

---

## Plans for the future
- [**🔑 Registration**] - User creates an account.
- [**🔐 Login**] - Receives a JWT token.
- [**📝 Add task**] - Title + description.
- [**📋 Task Listing**] - All tasks of the logged in user.
- [**❌ Delete Task**] - Deletes a task by ID.
- [**🖥️ GUI**] - Login screen, task list, add form.
- [**🖥️ DEPLOY**] - Deploy server to platform.

---

## Contact
<a href="https://discord.com/users/2023mipe" target="_blank">
  <img src="https://img.shields.io/badge/-Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white">
</a>
