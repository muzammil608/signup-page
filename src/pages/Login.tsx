import React, { useState } from "react";
import { Link, useNavigate } from "react-router-dom";

const Login: React.FC = () => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");
  const navigate = useNavigate();

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();

    const users = JSON.parse(localStorage.getItem("users") || "[]");
    const user = users.find(
      (u: any) => u.email === email && u.password === password
    );

    if (user) {
      setMessage("✅ Login successful! Redirecting...");
      localStorage.setItem("loggedInUser", JSON.stringify(user));
      setTimeout(() => navigate("/dashboard"), 1000); // redirect after success
    } else {
      setMessage("❌ Invalid email or password.");
    }
  };

  return (
    <form onSubmit={handleLogin} className="space-y-4 bg-white p-6 rounded shadow-md w-80">
      <h2 className="text-xl font-bold text-center">Login</h2>
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
        className="w-full bg-blue-500 text-white py-2 rounded hover:bg-blue-600"
      >
        Login
      </button>
      {message && <p className="text-center text-sm mt-2">{message}</p>}
      <p className="text-center text-sm mt-2">
        Don’t have an account?{" "}
        <Link to="/signup" className="text-green-500 hover:underline">
          Signup
        </Link>
      </p>
    </form>
  );
};

export default Login;
