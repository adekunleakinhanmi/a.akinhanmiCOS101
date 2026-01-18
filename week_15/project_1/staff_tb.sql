--
-- PostgreSQL database dump
--

\restrict zgLQxa0S5yKGKqW70loULMh3ckjF2bBJW2lcxccbXDNJ0XihP6phLuaWiXa32kG

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
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_eid_not_null NOT NULL,
    staff_name character(50) CONSTRAINT employees_ename_not_null NOT NULL,
    dno integer CONSTRAINT employees_dno_not_null NOT NULL,
    staff_sal real CONSTRAINT employees_esal_not_null NOT NULL,
    age integer CONSTRAINT employees_age_not_null NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
100	MUSTAPHA ALI                                      	3	175000	32	8047368945
107	ALOKWE MARTIN                                     	7	380000	48	9074867334
97	DANKODE AMINAT                                    	5	550000	40	7023435675
108	JOSIAH JOSHUA                                     	1	120000	30	8094857465
102	MAKINDE MARY                                      	2	450000	55	9034523478
120	ADELEKE JANE                                      	4	200000	38	7039487564
122	OSAHON MARK                                       	6	320000	44	8022938475
104	KUTI LAWAL                                        	1	750000	35	9145639484
117	SULEIMAN AJAYI                                    	3	800000	50	7030089981
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict zgLQxa0S5yKGKqW70loULMh3ckjF2bBJW2lcxccbXDNJ0XihP6phLuaWiXa32kG

