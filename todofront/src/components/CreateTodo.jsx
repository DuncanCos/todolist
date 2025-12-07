import React, { useState } from "react";
import api from "./api";

import { useAuth } from '@clerk/clerk-react'

export default function CreateTodo({ modal, closeModal, reseter }) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const { getToken } = useAuth();

  const createTodo = async () => {
    if (title == "" && description == "") {
      alert("cant create todo with nothing");
    } else {
      const token = await getToken()
      api
        .post("/todo/todo", {
          title,
          description,
        }, {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        })
        .then((response) => {
          reseter()
          cleanEnd();
        })
        .catch((error) => {
          alert("eror");
          console.log(error);
        });
    }
  };

  const cleanEnd = () => {
    setTitle("");
    setDescription("");
    closeModal();
  };

  if (!modal) return null;
  return (
    <div className="modal modal-open">
      <div className="modal-box flex justify-center">
        <div className="w-full text-center">
          <h1 className="text-4xl font-bold mb-6">Ajout ToDo</h1>
          <div className="flex flex-col items-center gap-3">
            <input
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              type="text"
              className="input w-1/2"
              placeholder="Titre"
            />
            <input
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              type="text"
              className="input w-1/2"
              placeholder="Description"
            />
          </div>
          <div className="m-4 flex justify-evenly">
            <button className="btn btn-error" onClick={cleanEnd}>
              annuler
            </button>
            <button className="btn btn-primary" onClick={createTodo}>
              confirmer
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
