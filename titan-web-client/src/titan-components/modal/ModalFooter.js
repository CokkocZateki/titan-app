import React from 'react'
import styled from 'styled-components'
import WithTheme from '../../titan-core/components/WithTheme'

export const FooterWrapper = styled.div`
  display: flex;
  flex-direction: row;
  padding: 10px 20px;
  border-top: ${props => props.borderColor};
  justify-content: flex-end;
`

class ModalFooter extends React.Component {
  render () {
    return (
      <FooterWrapper borderColor={this.props.titanTheme.palette.neutral}>
        {this.props.children}
      </FooterWrapper>
    )
  }
}

export default WithTheme(ModalFooter)
