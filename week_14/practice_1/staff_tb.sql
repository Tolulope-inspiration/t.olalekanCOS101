--
-- PostgreSQL database dump
--

\restrict qcKSWpXpLmQghI5r5Ihu9m80i1HhO59gczFZtCnhvB3eaLYGv2EY6XqrXrADBqU

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_e_id_not_null NOT NULL,
    staff_name character varying(50) CONSTRAINT employees_e_name_not_null NOT NULL,
    d_no integer CONSTRAINT employees_d_no_not_null NOT NULL,
    staff_sal real CONSTRAINT employees_e_sal_not_null NOT NULL,
    age integer CONSTRAINT employees_age_not_null NOT NULL,
    mobile character varying(15) CONSTRAINT employees_phone_not_null NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, d_no, staff_sal, age, mobile) FROM stdin;
100	Mustapha Ali	3	175000	32	8063285831
107	Alokwe Martin	7	380000	48	7090082812
97	Dankade Aminat	5	550000	40	9023688832
108	Josiah Joshua	1	120000	30	8053189131
102	Mankinde Mary	2	450000	55	9023487830
120	Adeleke Jane	4	200000	38	7061045862
122	Osahon Mark	6	320000	44	8022289842
117	Suleman Ajayi	3	800000	50	7030089811
104	Kuti Lawal	1	750000	35	9145689842
\.


--
-- PostgreSQL database dump complete
--

\unrestrict qcKSWpXpLmQghI5r5Ihu9m80i1HhO59gczFZtCnhvB3eaLYGv2EY6XqrXrADBqU

