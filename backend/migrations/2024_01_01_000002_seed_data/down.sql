-- Clean up seed data
DELETE FROM auth_users WHERE email IN ('test@example.com', 'admin@example.com');
DELETE FROM companies WHERE user_id IN (
    SELECT id FROM users WHERE username IN ('Bret', 'Antonette', 'Samantha', 'Karianne', 'Kamren', 'Leopoldo_Corkery', 'Elwyn.Skiles', 'Maxime_Nienow', 'Delphine', 'Moriah.Stanton')
);
DELETE FROM addresses WHERE user_id IN (
    SELECT id FROM users WHERE username IN ('Bret', 'Antonette', 'Samantha', 'Karianne', 'Kamren', 'Leopoldo_Corkery', 'Elwyn.Skiles', 'Maxime_Nienow', 'Delphine', 'Moriah.Stanton')
);
DELETE FROM users WHERE username IN ('Bret', 'Antonette', 'Samantha', 'Karianne', 'Kamren', 'Leopoldo_Corkery', 'Elwyn.Skiles', 'Maxime_Nienow', 'Delphine', 'Moriah.Stanton'); 