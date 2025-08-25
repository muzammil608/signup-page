import React, { useState } from "react";
import { Link } from "react-router-dom";

const Signup: React.FC = () => {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");

  const handleSignup = (e: React.FormEvent) => {
    e.preventDefault();

    const users = JSON.parse(localStorage.getItem("users") || "[]");
    const userExists = users.some((user: any) => user.email === email);

    if (userExists) {
      setMessage("❌ User already exists! Please login.");
    } else {
      const newUser = { firstName, lastName, email, password };
      users.push(newUser);
      localStorage.setItem("users", JSON.stringify(users));
      setMessage("✅ Signup successful! You can now login.");

      // clear inputs
      setFirstName("");
      setLastName("");
      setEmail("");
      setPassword("");
    }
  };

  return (
    <form onSubmit={handleSignup} className="space-y-4 bg-white p-6 rounded shadow-md w-80">
      <h2 className="text-xl font-bold text-center">Signup</h2>

      <input
        type="text"
        placeholder="First Name"
        value={firstName}
        onChange={(e) => setFirstName(e.target.value)}
        className="w-full p-2 border rounded"
        required
      />

      <input
        type="text"
        placeholder="Last Name"
        value={lastName}
        onChange={(e) => setLastName(e.target.value)}
        className="w-full p-2 border rounded"
        required
      />

      <input
        type="email"
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        className="w-full p-2 border rounded"
        required
      />

      <input
        type="password"
        placeholder="Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        className="w-full p-2 border rounded"
        required
      />

      <button
        type="submit"
        className="w-full bg-green-500 text-white py-2 rounded hover:bg-green-600"
      >
        Signup
      </button>

      {message && <p className="text-center text-sm mt-2">{message}</p>}

      <p className="text-center text-sm mt-2">
        Already have an account?{" "}
        <Link to="/login" className="text-blue-500 hover:underline">
          Login
        </Link>
      </p>
    </form>
  );
};

export default Signup;
