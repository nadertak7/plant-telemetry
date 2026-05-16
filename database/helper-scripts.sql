-- To exec into database service
-- docker compose exec -it postgres psql -U <user-name> -d <database>
-- For maintenece use user 'pg.super'.
-- !! Do not grant any non-super users priveleges to public when in database 'postgres'.

----------------------------
-- User/Role nanagement
----------------------------
-- Create readonly role and give them read access to all current and future tables
CREATE ROLE readonly_group;
GRANT CONNECT ON DATABASE plant_telemetry TO readonly_group;
GRANT USAGE ON SCHEMA public TO readonly_group;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO readonly_group; -- Current tables
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO readonly_group; -- Future tables

-- Create user and assign to role
CREATE ROLE <username> WITH LOGIN PASSWORD '<user-password>';
GRANT <role> TO <username>;
