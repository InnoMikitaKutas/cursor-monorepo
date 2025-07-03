import type { User } from '../types';

interface UserCardProps {
  user: User;
  onClick: () => void;
}

const UserCard = ({ user, onClick }: UserCardProps) => {
  return (
    <div 
      className="card p-6 cursor-pointer hover:shadow-lg transition-shadow"
      onClick={onClick}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          onClick();
        }
      }}
      aria-label={`View details for ${user.name}`}
    >
      <div className="flex items-start justify-between mb-3">
        <h3 className="text-lg font-semibold text-gray-800 truncate">
          {user.name}
        </h3>
        <div className="text-xs text-gray-500 bg-gray-100 px-2 py-1 rounded">
          @{user.username}
        </div>
      </div>
      
      <div className="space-y-2 text-sm text-gray-600">
        <div className="flex items-center">
          <svg className="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
          </svg>
          <span className="truncate">{user.email}</span>
        </div>
        
        {user.phone && (
          <div className="flex items-center">
            <svg className="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
            </svg>
            <span className="truncate">{user.phone}</span>
          </div>
        )}
        
        {user.address && (
          <div className="flex items-center">
            <svg className="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <span className="truncate">{user.address.city}</span>
          </div>
        )}
        
        {user.company && (
          <div className="flex items-center">
            <svg className="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
            </svg>
            <span className="truncate">{user.company.name}</span>
          </div>
        )}
      </div>
      
      <div className="mt-4 pt-3 border-t border-gray-200">
        <span className="text-xs text-primary-600 font-medium">
          Click to view details →
        </span>
      </div>
    </div>
  );
};

export default UserCard; 