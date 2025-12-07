import { useState, useEffect } from "react";
import ToDoCard from "./components/ToDoCard";
import UpdateTodo from "./components/UpdateTodo";
import api from "./components/api";
import CreateTodo from "./components/CreateTodo";

import { SignedIn, SignedOut, SignInButton, UserButton } from '@clerk/clerk-react';
import { useAuth } from '@clerk/clerk-react'

function App() {
  const [todos, setTodos] = useState([]);
  const [createModal, setCreateModal] = useState(false);
  const [reseter, setReset] = useState(false);
  const [updateModal, setUpdateModal] = useState(false);
  const [chosedTodo, setChosedTodo] = useState({});

  const getAllToDos = async () => {
    const token = await getToken()
    api.get("/todo/todo", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }).then((response) => {
      setTodos(response.data);
    });
  };

  useEffect(() => {
    getAllToDos();
  }, [reseter]);

  const { userId, sessionId, getToken, isLoaded, isSignedIn } = useAuth()

  const fetchExternalData = async () => {
    const token = await getToken()
    console.log(token)
  }

  useEffect(() => {
    fetchExternalData()
  }, [])

  if (!isLoaded) return <div>Loading...</div>

  return (
    <>
      <div className="flex justify-center  min-h-screen">

        <SignedOut>
          <SignInButton />
        </SignedOut>
        <SignedIn>
          <div>
            <header>
              <UserButton />
            </header>
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
                    chosingUpdated={(e) => setChosedTodo(e)}
                  />
                ))}
              </div>
            </div>
          </div>
        </SignedIn>


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
      </div>
    </>
  );
}

export default App;
