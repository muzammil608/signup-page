import React, { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";

const Login: React.FC = () => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate();

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setMessage("");

    try {
      const user: { id: number; email: string } = await invoke("login", {
        email,
        password,
      });

      setMessage("✅ Login successful! Redirecting...");
      localStorage.setItem("user", JSON.stringify(user));
      setTimeout(() => navigate("/dashboard"), 1000);
    } catch (error) {
      setMessage(`❌ ${error}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-100">
      <form onSubmit={handleLogin} className="space-y-4 bg-white p-6 rounded shadow-md w-80">
        <h2 className="text-xl font-bold text-center">Login</h2>
        <div>
          <input
            type="email"
            placeholder="Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            className="w-full p-2 border rounded"
            required
            disabled={isLoading}
          />
        </div>
        <div>
          <input
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full p-2 border rounded"
            required
            disabled={isLoading}
          />
        </div>
        <button
          type="submit"
          className="w-full bg-blue-500 text-white py-2 rounded hover:bg-blue-600 disabled:bg-blue-300"
          disabled={isLoading}
        >
          {isLoading ? "Logging in..." : "Login"}
        </button>
        {message && (
          <p className={`text-center text-sm mt-2 ${
            message.includes("✅") ? "text-green-600" : "text-red-600"
          }`}>
            {message}
          </p>
        )}
        <p className="text-center text-sm mt-2">
          Don't have an account?{" "}
          <Link to="/signup" className="text-green-500 hover:underline">
            Signup
          </Link>
          </p>
      </form>
    </div>
  );
};

export default Login;