import { useState, useEffect } from "react";
import ToDoCard from "./components/ToDoCard";
import UpdateTodo from "./components/UpdateTodo";
import api from "./components/api";
import CreateTodo from "./components/CreateTodo";

function App() {
  const [todos, setTodos] = useState([]);
  const [createModal, setCreateModal] = useState(false);
  const [reseter, setReset] = useState(false);
  const [updateModal, setUpdateModal] = useState(false);
  const [chosedTodo, setChosedTodo] = useState({});

  const getAllToDos = () => {
    api.get("/todo/todo").then((response) => {
      setTodos(response.data);
    });
  };

  useEffect(() => {
    getAllToDos();
  }, [reseter]);

  return (
    <>
      <div className="flex justify-center  min-h-screen">
        <div className="w-full max-w-2xl  text-center p-6 rounded-2xl shadow-lg">
          <h1 className="text-4xl font-bold mb-6">my todo list</h1>
          <div className="flex flex-col gap-4">
            <div
              className="btn btn-primary m-4"
              onClick={() => setCreateModal(!createModal)}
            >
              creer
            </div>

            {todos.map((todo) => (
              <ToDoCard
                key={todo.id}
                info={todo}
                reseter={() => setReset(!reseter)}
                updating={
                  () => setUpdateModal(!updateModal)
                }
                chosingUpdated={(e)=>setChosedTodo(e)}
              />
            ))}
          </div>
        </div>
      </div>
      <CreateTodo
        modal={createModal}
        closeModal={() => {
          setCreateModal(!createModal);
        }}
        reseter={() => setReset(!reseter)}
      />

      <UpdateTodo
        modal={updateModal}
        closeModal={() => {
          setUpdateModal(!updateModal);
        }}
        info={chosedTodo}
        reseter={() => setReset(!reseter)}
      />
    </>
  );
}

export default App;
