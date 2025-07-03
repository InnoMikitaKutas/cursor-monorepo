import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../contexts/AuthContext';

const Header = () => {
  const { user, logout } = useAuth();
  const navigate = useNavigate();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <header className="bg-white shadow-sm border-b border-gray-200">
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          <Link to="/" className="text-xl font-bold text-primary-600">
            JSONPlaceholder Clone
          </Link>
          
          <nav className="flex items-center space-x-4">
            {user ? (
              <>
                <span className="text-gray-600">
                  Welcome, {user.name}
                </span>
                <button
                  onClick={handleLogout}
                  className="btn-secondary"
                >
                  Logout
                </button>
              </>
            ) : (
              <>
                <Link to="/login" className="btn-primary">
                  Login
                </Link>
                <Link to="/register" className="btn-secondary">
                  Register
                </Link>
              </>
            )}
          </nav>
        </div>
      </div>
    </header>
  );
};

export default Header; 