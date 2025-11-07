import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";

import CreateTodo from "./components/CreateTodo";

function App() {
  const [count, setCount] = useState(0);
  const [createModal, setCreateModal] = useState(false);

  return (
    <>
      <div className="flex justify-center  min-h-screen">
        <div className="w-full max-w-2xl  text-center p-6 rounded-2xl shadow-lg">
          <h1 className="text-4xl font-bold mb-6">my todo list</h1>
          <div className="flex flex-col gap-4">
            <div className="btn btn-primary m-4" onClick={()=> setCreateModal(!createModal)}>creer</div>
            <div className="card  bg-base-300 card-md shadow-sm ">
              {" "}
              <div className="card-body">
                <h2 className="card-title">Medium Card</h2>
                <p>
                  A card component has a figure, a body part, and inside body
                  there are title and actions parts
                </p>
                <div className="justify-end card-actions">
                  <button className="btn btn-primary">Done</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <CreateTodo
        modal={createModal}
        closeModal={() => {
          setCreateModal(!createModal);
        }}
      />
    </>
  );
}

export default App;
