import { useState, useEffect } from 'react';
import { usersApi } from '../services/api';
import { useAuth } from '../contexts/AuthContext';
import UserCard from '../components/UserCard';
import UserModal from '../components/UserModal';
import type { User } from '../types';

const UsersList = () => {
  const { user: currentUser } = useAuth();
  const [users, setUsers] = useState<User[]>([]);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    loadUsers();
  }, []);

  const loadUsers = async () => {
    try {
      setIsLoading(true);
      const data = await usersApi.getUsers();
      setUsers(data);
    } catch {
      setError('Failed to load users');
    } finally {
      setIsLoading(false);
    }
  };

  const handleUserClick = (user: User) => {
    setSelectedUser(user);
  };

  const handleCloseModal = () => {
    setSelectedUser(null);
  };

  const handleDeleteUser = async (userId: string) => {
    try {
      await usersApi.deleteUser(userId);
      setUsers(users.filter(user => user.id !== userId));
      setSelectedUser(null);
    } catch {
      setError('Failed to delete user');
    }
  };

  if (!currentUser) {
    return (
      <div className="text-center py-12">
        <h1 className="text-2xl font-bold text-gray-800 mb-4">
          Welcome to JSONPlaceholder Clone
        </h1>
        <p className="text-gray-600 mb-6">
          Please login to view and manage users.
        </p>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="flex justify-center items-center py-12">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-500"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-12">
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded max-w-md mx-auto">
          {error}
        </div>
        <button 
          onClick={loadUsers}
          className="btn-primary mt-4"
        >
          Try Again
        </button>
      </div>
    );
  }

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold text-gray-800">Users</h1>
        <div className="text-sm text-gray-600">
          {users.length} users found
        </div>
      </div>

      {users.length === 0 ? (
        <div className="text-center py-12">
          <p className="text-gray-600">No users found.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {users.map((user) => (
            <UserCard
              key={user.id}
              user={user}
              onClick={() => handleUserClick(user)}
            />
          ))}
        </div>
      )}

      {selectedUser && (
        <UserModal
          user={selectedUser}
          onClose={handleCloseModal}
          onDelete={handleDeleteUser}
        />
      )}
    </div>
  );
};

export default UsersList; 