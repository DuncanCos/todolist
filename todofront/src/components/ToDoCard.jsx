import React from "react";
import api from "./api";

export default function ToDoCard({ info, reseter, updating, chosingUpdated }) {
  const todostyle = "card  bg-base-300 card-md shadow-sm";
  const donestyle = "card  bg-base-200 card-md shadow-sm";

  const tododone = () => {
    api
      .post(`/todo/todo/done/${info.id}`)
      .then((response) => {
        reseter();
      })
      .catch((error) => {
        alert("error done");
      });
  };
   const todoundone = () => {
    api
      .post(`/todo/todo/undone/${info.id}`)
      .then((response) => {
        reseter();
      })
      .catch((error) => {
        alert("error done");
      });
  };

  const removetodo = () => {
    api
      .delete(`/todo/todo/${info.id}`)
      .then((response) => {
        reseter();
      })
      .catch((error) => {
        alert("error done");
      });
  }


  const updateTodos = () => {
    chosingUpdated(info)
    updating()
  }

  return (
    <div className={info.status == "done" ? donestyle : todostyle}>
      {" "}
      <div className="card-body">
        <div className="flex justify-between">
          <div className="flex">
            <h2 className="">{info.title}</h2>
            <button onClick={updateTodos} className="btn btn-secondary">modif</button>
          </div>
          <button onClick={removetodo} className="btn btn-error">supprimer</button>
        </div>

        <p>{info.description}</p>
        <div className="justify-end card-actions">
          {info.status == "done" ? (
            <button onClick={todoundone} className="btn btn-neutral">
              unDo
            </button>
          ) : (
            <button onClick={tododone} className="btn btn-primary">
              Done
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
