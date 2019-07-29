import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import org.junit.After as After
import org.junit.runner.Request as Request
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

/*if (request == 'POST')
{
   
    POSTresponse=WS.sendRequestAndVerify(findTestObject('POST_api',[('url'):url, ('request'):request, ('responsebody'):responsebody,('requestbody'):requestbody,('statuscode'):statuscode, ('author'):author]))
    def POSTrestext = POSTresponse.getResponseText()

    println(POSTrestext)

    WS.verifyElementPropertyValue(POSTresponse, 'title', title)

   actualstatuscode = POSTresponse.statusCode
ResponseBodyjson=POSTresponse.getResponseBodyContent()
respos=ResponseBodyjson.toString()
substringabc=respos.substring(2,47)
println substringabc
aftertrim=substringabc.replace('  ', '')
//ResponseBodyjson.compareTo(responsebody)

WS.verifyEqual(aftertrim,responsebody)
   WS.verifyEqual(actualstatuscode, statuscode)

    WS.verifyElementPropertyValue(POSTresponse, 'author', author)
}

   if (request == 'PUT') {
	
	 PUTresponse=WS.sendRequestAndVerify(findTestObject('PUT_api',[('url'):url, ('request'):request,('responsebody'):responsebody,('requestbody'):requestbody,('statuscode'):statuscode, ('author'):author]))
	 def PUTrestext = PUTresponse.getResponseText()
 
	 println(PUTrestext)
 
	 WS.verifyElementPropertyValue(PUTresponse, 'title', title)
 
	 actualstatuscode = PUTresponse.statusCode
 
	 WS.verifyEqual(actualstatuscode, statuscode)
 
	 WS.verifyElementPropertyValue(PUTresponse, 'author', author)
 }
 
if (request == 'PATCH') 
{
	 PATCHresponse=WS.sendRequestAndVerify(findTestObject('PATCH_api',[('url'):url, ('request'):request,('responsebody'):responsebody,('requestbody'):requestbody,('statuscode'):statuscode, ('author'):author]))
	 def PATCHrestext = PATCHresponse.getResponseText()
 
	 println(PATCHrestext)
 
	 WS.verifyElementPropertyValue(PATCHresponse, 'title', title)
 
	 actualstatuscode = PATCHresponse.statusCode
 
	 WS.verifyEqual(actualstatuscode, statuscode)
 
	 WS.verifyElementPropertyValue(PATCHresponse, 'author', author)
 }*/
/*if (request == 'DELETE') {
	
	 DELETEresponse=WS.sendRequestAndVerify(findTestObject('DELETE_api',[('url'):url, ('request'):request,('responsebody'):responsebody,('requestbody'):requestbody,('statuscode'):statuscode, ('author'):author]))
	 def DELETErestext = DELETEresponse.getResponseText()
 
	 println(DELETErestext)
 
	 actualstatuscode = DELETEresponse.statusCode
 
	 WS.verifyEqual(actualstatuscode, statuscode)
 
 }*/

if (request == 'GET') {
    RequestObject requesturl = findTestObject('GET_api')

	//set the url for parameters
    requesturl.setRestUrl(url + "?title=$titlevalue")

	//send the request with parameters set in the above steps
    GETresponse = WS.sendRequest(requesturl)

    //GETresponse=WS.sendRequestAndVerify(findTestObject('GET_api',[('url'):url, ('titlevalue'):title,('request'):request,('responsebody'):responsebody,('requestbody'):requestbody,('statuscode'):statuscode, ('author'):author]))
    def GETrestext = GETresponse.getResponseText()

    println(GETrestext)

    actualstatuscode = GETresponse.statusCode

    WS.verifyEqual(actualstatuscode, statuscode)

    GETresponsebody = GETresponse.getResponseBodyContent()

    aftertrim = GETresponsebody.replace('  ', '')

    WS.verifyEqual(aftertrim, responsebody)
	
	
	
}

