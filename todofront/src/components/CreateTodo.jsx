import React, { useState } from "react";

export default function CreateTodo({ modal, closeModal }) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");

  const createTodo = () => {
    if (title == "" && description == "") {
      alert("cant create todo with nothing");
    } else {
      alert("todo creer " + title + " " + description);
      cleanEnd();
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
