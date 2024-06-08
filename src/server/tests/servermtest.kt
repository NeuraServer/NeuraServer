package com.neuraserver

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ServerManagerTest {

    @Test
    fun testStartAllInstances() {
        val serverManager = ServerManager()
        serverManager.startAllInstances()
        assertEquals(3, serverManager.runningInstancesCount)
    }

    @Test
    fun testStopAllInstances() {
        val serverManager = ServerManager()
        serverManager.startAllInstances()
        serverManager.stopAllInstances()
        assertEquals(0, serverManager.runningInstancesCount)
    }
}
