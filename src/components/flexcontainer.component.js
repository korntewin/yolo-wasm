import styled from 'styled-components';


const FlexContainer = styled.div`
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;

    > * {
    flex: 1 0 15em; // Adjust this value based on when you want to switch to one column
    margin: 0 10px;
    }
`

export default FlexContainer;