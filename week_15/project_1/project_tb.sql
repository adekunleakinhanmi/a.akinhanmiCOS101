--
-- PostgreSQL database dump
--

\restrict 78wjvE66U8VD0BPk0S1aSk7pLIWcnvqqzoUdqPzhUaIYsw0hhyImbzCsJkX6gN4

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

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
    pno integer NOT NULL,
    pname character(50) NOT NULL,
    pduration text NOT NULL,
    proj_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, proj_managerid) FROM stdin;
11	A                                                 	9 MONTHS	102
22	B                                                 	14 MONTHS	97
33	C                                                 	16 MONTHS	120
44	D                                                 	25 MONTHS	108
55	E                                                 	9 MONTHS	107
\.


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pname);


--
-- PostgreSQL database dump complete
--

\unrestrict 78wjvE66U8VD0BPk0S1aSk7pLIWcnvqqzoUdqPzhUaIYsw0hhyImbzCsJkX6gN4

