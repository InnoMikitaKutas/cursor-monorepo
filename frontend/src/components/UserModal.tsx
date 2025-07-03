import { useEffect } from 'react';
import type { User } from '../types';

interface UserModalProps {
  user: User;
  onClose: () => void;
  onDelete: (userId: string) => void;
}

const UserModal = ({ user, onClose, onDelete }: UserModalProps) => {
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('keydown', handleEscape);
    document.body.style.overflow = 'hidden';

    return () => {
      document.removeEventListener('keydown', handleEscape);
      document.body.style.overflow = 'unset';
    };
  }, [onClose]);

  const handleBackdropClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  const handleDelete = () => {
    if (window.confirm(`Are you sure you want to delete ${user.name}?`)) {
      onDelete(user.id);
    }
  };

  const getMapLink = () => {
    if (user.address?.geo) {
      return `https://www.google.com/maps?q=${user.address.geo.lat},${user.address.geo.lng}`;
    }
    return null;
  };

  return (
    <div 
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50"
      onClick={handleBackdropClick}
    >
      <div className="bg-white rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        <div className="sticky top-0 bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between">
          <h2 className="text-xl font-bold text-gray-800">User Details</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 transition-colors"
            aria-label="Close modal"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="p-6 space-y-6">
          {/* Basic Info */}
          <div>
            <h3 className="text-lg font-semibold text-gray-800 mb-3">Basic Information</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-600">Name</label>
                <p className="text-gray-800">{user.name}</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-600">Username</label>
                <p className="text-gray-800">@{user.username}</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-600">Email</label>
                <p className="text-gray-800">{user.email}</p>
              </div>
              {user.phone && (
                <div>
                  <label className="block text-sm font-medium text-gray-600">Phone</label>
                  <p className="text-gray-800">{user.phone}</p>
                </div>
              )}
              {user.website && (
                <div>
                  <label className="block text-sm font-medium text-gray-600">Website</label>
                  <a 
                    href={`https://${user.website}`} 
                    target="_blank" 
                    rel="noopener noreferrer"
                    className="text-primary-600 hover:text-primary-700 underline"
                  >
                    {user.website}
                  </a>
                </div>
              )}
            </div>
          </div>

          {/* Address */}
          {user.address && (
            <div>
              <h3 className="text-lg font-semibold text-gray-800 mb-3">Address</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-600">Street</label>
                  <p className="text-gray-800">
                    {user.address.street}
                    {user.address.suite && `, ${user.address.suite}`}
                  </p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-600">City</label>
                  <p className="text-gray-800">{user.address.city}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-600">Zipcode</label>
                  <p className="text-gray-800">{user.address.zipcode}</p>
                </div>
                {user.address.geo && (
                  <div>
                    <label className="block text-sm font-medium text-gray-600">Location</label>
                    <div className="flex items-center space-x-2">
                      <p className="text-gray-800 text-sm">
                        {user.address.geo.lat}, {user.address.geo.lng}
                      </p>
                      {getMapLink() && (
                        <a
                          href={getMapLink()!}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-primary-600 hover:text-primary-700 text-sm underline"
                        >
                          View on Map
                        </a>
                      )}
                    </div>
                  </div>
                )}
              </div>
            </div>
          )}

          {/* Company */}
          {user.company && (
            <div>
              <h3 className="text-lg font-semibold text-gray-800 mb-3">Company</h3>
              <div className="space-y-2">
                <div>
                  <label className="block text-sm font-medium text-gray-600">Name</label>
                  <p className="text-gray-800">{user.company.name}</p>
                </div>
                {user.company.catch_phrase && (
                  <div>
                    <label className="block text-sm font-medium text-gray-600">Catchphrase</label>
                    <p className="text-gray-800 italic">"{user.company.catch_phrase}"</p>
                  </div>
                )}
                {user.company.bs && (
                  <div>
                    <label className="block text-sm font-medium text-gray-600">Business</label>
                    <p className="text-gray-800">{user.company.bs}</p>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>

        <div className="sticky bottom-0 bg-gray-50 border-t border-gray-200 px-6 py-4 flex justify-between">
          <button
            onClick={handleDelete}
            className="bg-red-500 hover:bg-red-600 text-white font-medium py-2 px-4 rounded-lg transition-colors"
          >
            Delete User
          </button>
          <button
            onClick={onClose}
            className="btn-secondary"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};

export default UserModal; 