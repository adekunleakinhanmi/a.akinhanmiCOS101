--
-- PostgreSQL database dump
--

\restrict LFdXtK7R67MOHFdJlIEHUmbVN10KkRuyAg5g8DwCTbhcEyoIjbhrl2bJXqpPoad

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
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

\unrestrict LFdXtK7R67MOHFdJlIEHUmbVN10KkRuyAg5g8DwCTbhcEyoIjbhrl2bJXqpPoad

