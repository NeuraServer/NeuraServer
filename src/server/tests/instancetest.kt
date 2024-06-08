package com.neuraserver

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ServerInstanceTest {

    @Test
    fun testServerInstanceStart() {
        val serverInstance = ServerInstance(5500)
        serverInstance.start()
        assertEquals(true, serverInstance.isRunning)
    }

    @Test
    fun testServerInstanceStop() {
        val serverInstance = ServerInstance(5500)
        serverInstance.start()
        serverInstance.stop()
        assertEquals(false, serverInstance.isRunning)
    }
}
