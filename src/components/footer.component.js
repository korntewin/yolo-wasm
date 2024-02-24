import styled from 'styled-components';

const FooterContainer = styled.footer`
  padding: 20px;
  display: flex;
  justify-content: flex-end;
  align-items: flex-end;
`;

const Section = styled.div`
  margin-right: 10px;
  margin-left: 10px;
`;

const Content = styled.b`
  font-size: 0.8em;
`;

const Footer = () => (
    <FooterContainer>
        Please follow me here 👉
        <Section>
            <Content>
                <a
                    href="https://github.com/korntewin"
                    target="_blank"
                    rel="noreferrer"
                >
                    Github
                </a>
            </Content>
        </Section>
        |
        <Section>
            <Content>
                <a href="https://www.linkedin.com/in/korntewin/" target="_blank" rel="noreferrer">
                    Linkedin
                </a>
            </Content>
        </Section>
        👈
    </FooterContainer>
);

export default Footer;