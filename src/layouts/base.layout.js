import FooterComponent from "../components/footer.component";


const Layout = ({ children }) => {
    return (
        <>
            <header className="App-header">
            {children}
            <FooterComponent />
            </header>
        </>
    )
}

export default Layout;