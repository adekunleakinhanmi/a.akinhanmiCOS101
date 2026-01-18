--
-- PostgreSQL database dump
--

\restrict ErC5LEN4YWD41IPClYY7hreZHDaX1lz4rNmvaYZUqaC2aAmcv0JLek6yBCF1nWK

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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile character varying(15) NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size text NOT NULL,
    data_duration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname character(50) NOT NULL,
    dlocation text NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

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
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	MUSTA KARIM	35	m_karim@gmail.com	8055089112	102	5
111	LILIAN JAIYE	43	l_jaiye@gmail.com	8055185341	100	3
112	ARTHUR MUSA	50	a_musa@gmail.com	7055282813	107	10
113	PHILIP AKONJO	41	p_akonjo@gmail.com	9027836475	100	2
114	MARYLENE MAPA	33	m_mapa@gmail.com	8054333475	120	5
115	OGHENERO AGOR	50	o_agor@gmail.com	7055566774	117	11
116	ADAMS BREE	33	a_bree@gmail.com	8056765424	102	1
117	OKAFOR MATHIAS	45	o_mathias@gmail.com	8056763367	120	10
118	SAMSON ADELEKE	65	s_adeleke@gmail.com	7056774423	117	11
119	LAWAL TAMIRE	35	l_tamire@gmail.com	9052111101	107	5
120	JAMES JOB	44	j_job@gmail.com	8069578634	10	8
121	MATTHEW JAKANDE	21	m_jakande@gmail.com	7024583943	120	2
122	JIMILA ADEGBOYE	20	j_adegboye@gmail.com	8097584322	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3,9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	ADMINISTRATION                                    	IKEJA	44
101	2	ACCOUNT                                           	EGBEDA	11
100	3	PACKAGING                                         	AJAH	44
120	4	RESEARCH                                          	V.I	33
97	5	ACCOUNT                                           	MAGODO	22
122	6	OPERATIONS                                        	MILE 2	44
107	7	PACKAGING                                         	KETU	55
\.


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
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dno);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pname);


--
-- PostgreSQL database dump complete
--

\unrestrict ErC5LEN4YWD41IPClYY7hreZHDaX1lz4rNmvaYZUqaC2aAmcv0JLek6yBCF1nWK

