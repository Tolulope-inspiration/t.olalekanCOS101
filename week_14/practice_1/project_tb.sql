--
-- PostgreSQL database dump
--

\restrict 6zJj1oPdciuAmtLnfYUQU9poAWz93hTkEtKEMvhjLMV4l5QDtF6EYHrmqeRR6Wf

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
-- PostgreSQL database dump complete
--

\unrestrict 6zJj1oPdciuAmtLnfYUQU9poAWz93hTkEtKEMvhjLMV4l5QDtF6EYHrmqeRR6Wf

