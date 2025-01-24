
---

# RustyTodo  

**RustyTodo** is a high-performance task management application built with **Rust** for the backend and **Next.js** for the frontend. This project combines the speed and safety of Rust with the server-rendering capabilities of Next.js, delivering an efficient and intuitive experience for managing todos.  

---

## Features  

- **High Performance**: Backend built with Rust's Actix Web framework for blazing-fast API responses.  
- **Server-Side Rendering (SSR)**: Next.js ensures faster load times and improved SEO.  
- **Full Task Management**: Add, edit, delete, search, and mark tasks as completed.  
- **Responsive Design**: Works seamlessly across devices.  
- **Dynamic Routing**: Efficient routing for individual tasks and categories.  

---

## Technologies Used  

### Backend  
- **Language**: Rust  
- **Framework**: Actix Web  
- **Database**: Mutex for in-memory storage (extendable to databases like PostgreSQL).  

### Frontend  
- **Framework**: Next.js (React-based)  
- **Styling**: Custom CSS   
- **API Communication**: Axios for REST API calls  

---

## Installation  

### Prerequisites  
Ensure you have the following installed:  
- Rust (latest stable version)  
- Node.js (16.x or later)  
- npm or yarn  

### Backend Setup  
1. Clone the repository:  
   ```bash  
   git clone https://github.com/your-username/RustyTodo.git  
   cd RustyTodo/backend  
   ```  
2. cd into the rust folder and run the Rust server:  
   ```bash  
   cargo run  
   ```  
3. The server will start on `http://127.0.0.1:8080`.  

### Frontend Setup  
1. Navigate out to the main folderthe frontend directory:  
   Install dependencies:  
   ```bash  
   npm install  
   ```  
2. Start the development server:  
   ```bash  
   npm run dev  
   ```  
3. Open your browser and go to `http://localhost:3000`.  

---

## API Endpoints  

### Todos API  
- **GET /todos**: Retrieve all todos.  
- **POST /todos**: Create a new todo.  
  - **Body**: `{ "title": "string", "completed": false }`  
- **PUT /todos/{id}**: Update a todo by ID.  
  - **Body**: `{ "title": "string", "completed": true/false }`  
- **DELETE /todos/{id}**: Delete a todo by ID.  

---

## Usage  

1. Open the app in your browser.  
2. Add tasks using the input box and "Add" button.  
3. Edit tasks by clicking the edit icon.  
4. Mark tasks as completed by checking the box.  
5. Search for specific tasks using the search bar.  

---

## Contributing  

We welcome contributions! To get started:  
1. Fork the repository.  
2. Create a new branch: `git checkout -b feature-name`.  
3. Commit your changes: `git commit -m "Add feature-name"`.  
4. Push to the branch: `git push origin feature-name`.  
5. Open a pull request.  

---

## License  

This project is licensed under the [MIT License](LICENSE).  

---

## Contact  

For questions or support, reach out to:  
**Your Name**  Kazeem Lawal
**Email**: quazeemastic@gmail.com  
**GitHub**: [lawalstacks](https://github.com/lawalstacks)  

---  

**RustyTodo**: Manage your tasks efficiently with the power of Rust and Next.js! 🚀  