--
-- PostgreSQL database dump
--

\restrict h2qFh6OKVQIWu9Qh6tfoVJeuhIFVHCV8PTK8RakTZYlACyzweJghlow6POWjWbO

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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_manager_id integer NOT NULL,
    d_no integer NOT NULL,
    d_name character varying(50) NOT NULL,
    d_location character varying(60) NOT NULL,
    p_no integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    p_no integer NOT NULL,
    p_name character varying(10) NOT NULL,
    p_duration text NOT NULL,
    project_manager_id integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

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
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_manager_id, d_no, d_name, d_location, p_no) FROM stdin;
108	1	Administration	Ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	V.I	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (p_no, p_name, p_duration, project_manager_id) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


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

\unrestrict h2qFh6OKVQIWu9Qh6tfoVJeuhIFVHCV8PTK8RakTZYlACyzweJghlow6POWjWbO

