import React, { Component } from 'react'
import Table from '../../titan-components/table/Table'
import TableHeader from '../../titan-components/table/TableHeader'
import TableRowCell from '../../titan-components/table/TableRowCell'
import TableBody from '../../titan-components/table/TableBody'
import TableRow from '../../titan-components/table/TableRow'
import { ContentBlock } from 'titan-components'

class BranchOverviewScene extends Component {
  render () {
    return (
      <ContentBlock>
        <h1>Branch</h1>
        <Table height={300}>
          <TableHeader>
            <TableRowCell>Rank</TableRowCell>
            <TableRowCell>Name</TableRowCell>
            <TableRowCell>Last Online</TableRowCell>
            <TableRowCell>Actions</TableRowCell>
          </TableHeader>
          <TableBody>
            <TableRow>
              <TableRowCell width={'50px'}>1</TableRowCell>
              <TableRowCell>Wrapping not allowed, so this will be truncated.</TableRowCell>
              <TableRowCell>SSgt.</TableRowCell>
              <TableRowCell>None</TableRowCell>
            </TableRow>
            <TableRow>
              <TableRowCell width={'50px'}>2</TableRowCell>
              <TableRowCell allowWrapping>Wrapping is allowed, so this will not be truncated.</TableRowCell>
              <TableRowCell>SSgt.</TableRowCell>
              <TableRowCell>None</TableRowCell>
            </TableRow>
          </TableBody>
        </Table>
      </ContentBlock>
    )
  }
}

export default BranchOverviewScene
